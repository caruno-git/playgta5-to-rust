# Correct SIMD zero-extending loads

Author: carunodev @stfu

Both translation backends treated `v128.load32_zero` and `v128.load64_zero` as
splat loads. These instructions must read four or eight bytes into the low part
of the vector and set its remaining bytes to zero. Repeating the loaded value
instead corrupted later vector operations.

For example, the cloth velocity packer builds masks intended only for the W
component. The repeated value also set bits in position Z: packing zero velocity
changed Z=2 to approximately 3.92e19. The corrected scalar unsigned load followed
by a `u128` cast preserves XYZ and the original packed W value. Genuine splat
instructions continue to use the existing broadcast operation.

The generator and fallback now honor the zero-extension flag. This update also
corrects 5,533 verified expressions in 1,722 delivered functions. Each expression
was matched to its original instruction; load addresses, widths, genuine splats
and code outside those expressions were preserved. Existing rendering, readback
lifetime and grass-pool corrections remain in place.

Run the portable checks with Python 3.9+, Node.js, Cargo and rustc:

```text
python -m unittest discover -s tools -p "test_simd_memory_loads.py" -v
python -m unittest discover -s tools -p "test_simd_zero_pack_delivery.py" -v
```

The load tests execute both generated backends against actual WebAssembly SIMD
instructions, checking zero upper lanes, real splats, unaligned/static-offset
loads, exact memory-end access and out-of-bounds traps. The packing regression
compiles the actual delivered function and an exact pre-fix source fixture. It
reproduces the old corruption and checks 2,069 synthetic cases in both debug and
release builds, including XYZ bit preservation, unchanged W and neighboring
memory canaries. No original binary, game assets or unpublished capture is
needed to run these tests.

The regular engine built as WASM SHA-256
`a4922b4b6e45fb4ace72f5ab0154fe2c611b8051bca8f08f6c3baf9575138dbe`
(95,868,954 bytes). Its 89 named imports, 1,995 public exports/signatures and
memory/table declarations match the preceding release.

Two separate Michael gameplay captures now show all 130 native and physical GPU
cloth positions finite, where the broken baseline had none. For the same
13,782-index mesh, invalid cloth vertices fell from 929 to zero and triangles
containing them from 1,447 to zero. Actual vertex/index bytes and native rest,
binding, bone-map and edge-constraint bytes match the baseline exactly. The full
17,216-byte GPU uniform pack matches its same-draw CPU source, and saved-image
review shows the complete shirt.

The strict full-outfit comparison still fails: component 11, drawable 2 selected
texture 2 instead of texture 3 during the normal character-switch workflow.
Primary upper component 3 remains drawable 8/texture 0. Texture descriptors for
the diagnosed draw match, but their payload bytes were not captured; matching
descriptors do not prove matching textures. The unchanged mesh and simulation
inputs support the numerical cloth correction, not an exact full-outfit visual
comparison or proof that the source change cannot affect outfit selection.

A separate run with the normal WASM compiler, 16 reported cores and one WASM
compilation task completed 35 seconds of streaming movement and six fullscreen
cycles: 18 actual size changes across 2560x1600, 2560x1398 and 1638x1096. It kept
presenting frames and moving the player, without panic, GPU validation or HTTP
errors. The native full map and zoom also passed a separate visual check at
2560x1600.

A wider sequential character run passed Franklin and Michael gameplay checks.
Michael was in a vehicle and moved 6.3 metres using normal S input; all 195
captured native cloth points were finite. The Trevor transition started but
did not reach gameplay within 90 seconds: the observed model remained Michael
at LOD 3 with the camera about 1,153 metres away. The run recorded no engine
panic or GPU validation errors, and at least 2.58 GB of host memory remained
free. This is an unresolved streaming/character-switch limitation; it is not
a passing three-character result.

Cold loading remains a limitation: the first attempt exceeded the 160-second
world-ready deadline before gameplay; a subsequent full WASM download took
126 seconds. Later attempts using cached resources reached gameplay. These
observations are not a startup-speed improvement claim. A broader run confirmed
Franklin and a second Michael outfit with 195 finite cloth points; both remained
controllable and continued rendering. The subsequent Trevor transition did not
finish within 90 seconds, so that sequence is not a three-character pass.
Character-switch streaming and long-session memory/stability remain follow-ups. This update
does not establish that every rendering, crash, memory-growth or loading issue
is resolved, and includes no compiled engine, experimental diagnostic feature,
shader replacement or game assets.
