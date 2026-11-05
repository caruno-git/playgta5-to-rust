# SIMD zero-extending load correction — November 4, 2026

Author: carunodev @stfu

This correction fixes invalid cloth geometry caused by incorrectly translated
SIMD loads. It is included in the
[November 5 browser release](https://github.com/caruno-git/playgta5-to-rust/releases/tag/v2026.11.05-rust-web).

## Cause and correction

Both translation backends treated `v128.load32_zero` and `v128.load64_zero`
as splat loads. These instructions must load four or eight bytes into the low
part of a vector and clear the remaining bytes. Repeating the loaded value
instead corrupted later vector operations.

The cloth velocity packer illustrates the effect. A mask intended for the W
component also changed bits in position Z: packing zero velocity changed Z=2
to approximately 3.92e19. A scalar unsigned load followed by a `u128` cast
implements the required zero extension while preserving XYZ and the packed W
value. Genuine splat instructions retain their broadcast behavior.

The generator and fallback now honor the zero-extension flag. The change
corrected 5,533 verified expressions in 1,722 translated functions, matching
each expression to its original instruction. Load addresses, widths, genuine
splats and unrelated expressions were preserved, along with the existing
rendering, readback-lifetime and grass-pool fixes.

## Regression coverage

The portable checks require Python 3.9 or newer, Node.js, Cargo and `rustc`:

```text
python -m unittest discover -s tools -p "test_simd_memory_loads.py" -v
python -m unittest discover -s tools -p "test_simd_zero_pack_delivery.py" -v
```

The load tests compare both generated backends with executed WebAssembly SIMD
instructions. Cases cover zero upper lanes, genuine splats, unaligned and
static-offset loads, exact memory-end access and out-of-bounds traps.

The packing regression compiles the tracked function and an exact pre-fix
fixture. It reproduces the corruption and checks 2,069 synthetic cases in debug
and release builds, including bit-for-bit XYZ preservation, unchanged W and
neighboring-memory canaries. The tests require no original binary, game assets
or external gameplay captures.

## Engine and cloth validation

The corrected engine is 95,868,954 bytes, SHA-256:

```text
a4922b4b6e45fb4ace72f5ab0154fe2c611b8051bca8f08f6c3baf9575138dbe
```

Its 89 named imports, 1,995 public exports and signatures, and memory/table
declarations match the preceding release.

Two Michael gameplay captures showed all 130 native and physical GPU cloth
positions finite; the broken baseline had none. On the same 13,782-index mesh,
invalid cloth vertices fell from 929 to zero, and triangles containing them
fell from 1,447 to zero. Vertex/index bytes and native rest positions, bindings,
bone maps and edge constraints matched the baseline exactly. The 17,216-byte
GPU uniform pack matched its same-draw CPU source, and image review showed a
complete shirt.

Full-outfit equivalence was not established. Component 11, drawable 2 selected
texture 2 instead of texture 3 during normal character switching. The primary
upper-body component 3 remained drawable 8/texture 0. Texture descriptors for
the diagnosed draw matched, but texture payload bytes were not captured.
The matching mesh and simulation inputs support the numerical cloth fix;
they do not prove identical outfit appearance or exclude an effect on outfit
selection.

## Other gameplay checks and remaining limits

A separate run used the normal WASM compiler, 16 reported cores and one WASM
compilation task. It completed 35 seconds of movement with streaming and six
fullscreen cycles: 18 size changes across 2560×1600, 2560×1398 and 1638×1096.
The player remained controllable and frames continued to render without a
recorded panic, GPU validation error or HTTP error. The full map and zoom also
passed a separate visual check at 2560×1600.

A sequential character test reached Franklin and Michael gameplay. Michael
moved 6.3 metres in a vehicle using the normal reverse input, and all 195
captured native cloth points in that outfit were finite. The Trevor transition
did not reach gameplay within 90 seconds: the observed model remained Michael
at LOD 3, with the camera about 1,153 metres away. No engine panic or GPU
validation error was recorded, and at least 2.58 GB of host memory remained
free. This does not constitute a passing three-character or driving-stability
test.

Cold loading also remained slow. One attempt exceeded a 160-second world-load
deadline; a subsequent full WASM download took 126 seconds. Later attempts
reached gameplay with cached resources. These measurements do not establish a
startup-speed improvement.

Character-switch streaming, cold-load performance, memory growth and
long-session stability remain open validation areas. The cloth correction does
not establish that every rendering or stability issue is resolved.
