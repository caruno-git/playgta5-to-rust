# Browser asset streaming

The [IO worker](../browser/io_worker.js) loads game files over HTTP and serves
engine reads through shared WebAssembly memory. It caches downloaded blocks in
the browser's Origin Private File System (OPFS). When persistent storage is
unavailable, it uses a 48 MiB memory cache.

The worker is included in the
[2026.11.05 browser release](https://github.com/caruno-git/playgta5-to-rust/releases/tag/v2026.11.05-rust-web),
alongside the compiled engine and browser host. The original game data remains
an external requirement. See the [README](../README.md) for installation and
[release status](../linked/STATUS.md) for gameplay limitations.

## Request priorities

The engine sends two kinds of work: a demand read is needed immediately, while
a hint identifies data expected to be needed later. The worker also performs
speculative read-ahead and startup prefetching.

The previous limit counted hint jobs. Each ordinary 1 MiB hint could start
eight 128 KiB HTTP transfers, so 96 jobs could create 768 requests ahead of a
blocking engine read. The current scheduler limits the transfers themselves.

| Work | Admission and HTTP priority |
| --- | --- |
| Demand read | High HTTP priority; eight of the 96 transfer slots are reserved for demand. |
| Normal hint | Uses the remaining capacity and low HTTP priority. A queued hint needed by a demand is promoted in place without a duplicate transfer. |
| Speculative work | Low HTTP priority; shares background capacity and has an additional 48-transfer cap. |

Normal archive hints retain their 128 KiB slices. Untouched compressible files
can be fetched whole, up to 8 MiB. Large speculative reads retain the existing
separate urgent fallback and recovery behavior.

Once a normal hint is already fetching, this implementation cannot change its
HTTP priority. A demand for the same slice joins that transfer. The scheduler
therefore helps newly issued demands and queued hints more directly than it
helps demands joining a transfer already in progress.

Blocks enter the persistent index only after complete writes. Independent
slices can complete separately, and shared-memory views refresh after
asynchronous work. The change preserves the existing asset bytes, graphics
settings, read-ahead distances, cache format and native collision-loading
barriers. The memory-cache fallback still reads its requested blocks
sequentially.

## Server configuration

The hosted release uses existing gzip sidecars for whole-file requests with
these metadata and effect-file extensions:

```text
fxc meta xml dat ymt txt csv gfx sps cfg
```

The same files must retain raw byte offsets for partial reads. A compatible
host implements the following response contract:

| Request | Response |
| --- | --- |
| Whole file, gzip accepted and a valid sidecar exists | Gzip body that decodes to the original file. |
| Whole file, identity requested or sidecar absent | Original file bytes. |
| Valid byte range, including requests accepting gzip | Uncompressed `206` with the exact `Content-Range` and bytes. |
| Unsatisfiable byte range | `416` with the original file length. |
| RPF archive range | Direct, uncompressed static range response. |

For Nginx, range requests must reach the raw range handler before
`gzip_static` selects a sidecar. The metadata location must take precedence
over the general `/data/` location. Cross-origin isolation headers and the
appropriate cache policy must also apply to these responses. The JS, JSON and
WASM compression rules remain separate.

Validation compared 3,560 metadata gzip files with their decoded originals:
378,665,988 raw bytes and 58,965,987 gzip bytes. Two missing sidecars used raw
fallback. Public checks covered identity, gzip and `gzip;q=0`, prefix and suffix
ranges, `416` responses and the archive-range path. This change requires no
dynamic compression or modification of game assets.

Each deployed JavaScript file and its `.gz` sidecar must contain matching code.
A stale sidecar can cause clients accepting gzip to execute a different worker.
The current release uses a new versioned build directory so existing HTTP cache
entries cannot mix its runtime files with an earlier release.

## Tests

The portable worker tests require Node.js and its built-in modules. Run from
the repository root:

```sh
node --test tools/test_streaming_worker.cjs
```

The tests execute the worker with controlled network responses and the actual
shared-memory request protocol. They check transfer limits, HTTP priorities,
promotion without duplication, independent slice completion, cache reuse,
speculative fallback, required-read failures and shared-memory growth.

The published source is 45,155 bytes, with SHA-256:

```text
9f5d4262c699ef8490dee16aca9b4cfa3e2d26e1d5c17cfa58f1d8e9c40103d6
```

The scoped [Git attributes](../browser/.gitattributes) preserve the file's exact
bytes across checkouts, and the tests verify this identity.

## Measured behavior

A browser HTTP/2 test issued sixteen 1 MiB hints, followed by an independent
128 KiB demand. Every accepted run completed 129 exact ranges, indexed 4,128
blocks and received 16.125 MiB. The demand bytes matched an independently
downloaded SHA-256 reference, and all queues drained.

| Worker | Demand latency | All background hints finished |
| --- | ---: | ---: |
| Previous worker, two runs | 4.145–5.032 s | 4.251–5.116 s |
| Transfer limit alone, one run | 5.129 s | 6.935 s |
| Current worker, two runs | 0.745–0.793 s | 6.257–7.082 s |

The current worker completed the blocking demand sooner in these runs while
taking longer to finish the background queue. A further check of the deployed
worker passed the same integrity and completion requirements with a demand
latency of 3.103 seconds.

Network conditions and upstream cache state were uncontrolled. This test
measures HTTP contention; it does not include native decoding, resource
placement or driving. Its demand range had not been hinted. The results do not
establish a fixed speedup, a general throughput gain or an FPS improvement.

Driving stability remains a separate validation task. Remaining stalls may
involve new demands, queued hints or joins of active transfers. The transport
also has no explicit response deadline, so this scheduler does not provide
recovery from an indefinitely stalled connection.
