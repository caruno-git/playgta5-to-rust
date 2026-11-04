# Browser streaming worker

Author: carunodev @stfu

`browser/io_worker.js` is the browser IO worker used with the Rust engine's
existing shared-memory file protocol. It requires the existing loader, asset
manifest and game files. Those inputs are not included here. This file is not a
replacement for the engine, renderer or complete browser host.

The published source matches the tested worker byte for byte:

```text
SHA-256: 9f5d4262c699ef8490dee16aca9b4cfa3e2d26e1d5c17cfa58f1d8e9c40103d6
Bytes: 45155
```

The scoped `browser/.gitattributes` preserves its existing mixed line endings
so checkout normalization does not change that identity. No Rust build default
or other repository text setting changes.

## Scheduling behavior

Previously, the 96-hint limit applied to jobs. Each ordinary 1 MiB hint could
start eight 128 KiB transfers, allowing 768 HTTP requests ahead of a blocking
engine read. The worker now limits actual data-transfer operations to 96 and
reserves eight admissions for demand reads. Normal hints and speculative work
share the remaining capacity; speculative operations also retain their 48-slot
cap.

Normal hints and speculative transfers use low HTTP Fetch priority. A direct
demand uses high priority. If a demand joins a normal hint still waiting in the
worker queue, that queued transfer moves to demand priority in place and starts
with high HTTP priority. It does not create a second transfer of the same slice.

An already-running normal hint cannot have its Fetch priority changed through
this implementation. A joining demand waits for that transfer. Normal archive
hints retain their 128 KiB slices. Untouched compressible files retain the
existing whole-file path, up to 8 MiB. Speculative large reads retain the separate
urgent retry and failure-recovery behavior.

Persistent-cache blocks are still indexed only after complete writes. Individual
normal slices can complete without waiting for unrelated slices. Shared-memory
views refresh after asynchronous work. Graphics settings, assets, read-ahead
distances, native collision barriers, block sizes and the 48 MiB fallback cache
remain unchanged. The experimental parallel fallback for unavailable OPFS is
not included.

## Metadata compression

The matching test deployment serves existing gzip sidecars for whole-file
requests under `/data/` with these extensions:

```text
fxc meta xml dat ymt txt csv gfx sps cfg
```

Its existing JS/JSON/WASM compression path remains separate. The added metadata
location precedes that existing regex location and has the following behavior:

```nginx
location ~* ^/data/.*\.(?:fxc|meta|xml|dat|ymt|txt|csv|gfx|sps|cfg)$ {
    if ($http_range != "") {
        return 418;
    }
    try_files $uri =404;
    gzip_static on;
}
```

This excerpt depends on the existing server's `error_page 418 = @lab_code_range`
mapping and raw range handler; it is not a standalone server configuration.
Range requests use raw 206/416 behavior even when the client accepts gzip.
RPF archive ranges remain direct, uncompressed static responses. No dynamic
compression or asset rewrite is introduced. Missing sidecars fall back to the
raw file, and cross-origin isolation and cache headers remain inherited.

Sidecar validation decoded and compared 3,560 existing metadata gzip files
against their raw originals: 378,665,988 raw bytes versus 58,965,987 gzip bytes.
Two missing sidecars retained raw fallback. Public protocol checks cover gzip
decoded bytes, identity and gzip-q0 responses, raw prefix and suffix ranges,
416 responses and the unchanged archive range path. Deployment-specific
configuration generators, credentials and service-management scripts are not
part of this source snapshot.

When installing a new worker in the existing host, keep any served `.gz` copy
consistent with the raw JavaScript. A stale compressed sidecar can otherwise
serve different code from the visible source.

## Verification and limits

Portable tests need Node.js built-ins only, without game assets or a browser:

```text
node --test tools/test_streaming_worker.cjs
```

They execute the published worker with held synthetic network responses and the
real shared-memory request protocol. They verify actual transfer admission,
normal and demand HTTP priorities, queued promotion without duplication,
independent slice completion, cache reuse, speculative fallback, required-read
failure and shared-memory growth. The source identity is pinned by a separate
test.

Five actual browser HTTP/2 contention runs used sixteen 1 MiB hints followed by
one independent 128 KiB demand. All completed 129 exact ranges, indexed 4,128
blocks and received 16.125 MiB. The urgent bytes matched an independently
downloaded SHA-256 reference, all queues drained and owned browsers closed.

| Worker | Urgent read | All background hints finished |
| --- | ---: | ---: |
| Previous worker, two runs | 4.145–5.032 s | 4.251–5.116 s |
| Admission limit alone, one run | 5.129 s | 6.935 s |
| Published worker, two runs | 0.745–0.793 s | 6.257–7.082 s |

The published worker prioritized the blocking demand at the cost of a slower
complete background queue in this fixture. The two-run descriptive median
urgent latency was about six times lower; this is not a general throughput or
FPS result. Client caches were fresh, while upstream cache state and link
conditions were uncontrolled. The fixture used no engine, native decoding,
placement or driving workload, and its demand range had not been hinted.

After this worker became the test site's default, a further public IO-only
smoke run passed the same 129-range, 4,128-block, byte-count, urgent-hash and
queue-drain checks. Its urgent read took 3.103 s. This additional observation
shows the network variability and does not support a guaranteed sixfold gain.

Driving acceptance remains separate. In particular, measurements must
distinguish cold demands, promotion of queued hints and demands joining active
hints before attributing a remaining stall to network priority. The inherited
transport still has no explicit response deadline, and this change does not
claim to recover indefinitely stalled connections or fix every driving freeze.
