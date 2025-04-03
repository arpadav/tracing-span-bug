# `tracing` bug with spans

For projects which have:

* a default logger using `tracing`, initialized on startup
* followed by a local `tracing` logger

Then, performing a `reqwest::Client::get` will cause a panic within any of the async runtimes. Why? Not sure. This is a problem with `tracing`

However, problem is fixed indirectly by removing any `tracing` spans created/entered in the `hyper-util` crate (dependency of `reqwest`).

## Tests

See: https://github.com/arpadav/tracing-span-bug/blob/main/hyper-util-dns-with-span/src/lib.rs
See: https://github.com/arpadav/tracing-span-bug/blob/main/hyper-util-dns-without-span/src/lib.rs

The files should be identical. The only change is the removal of the `tracing::debug_span!` call from a patched `hyper-util` crate.

## What do you mean by `span`

Only code changes in `hyper-util`: https://github.com/hyperium/hyper-util/pull/179/files

This removes the `tracing::debug_span!` call, and does not enter it upon spawning a `tokio::task::spawn_blocking` task. This issue should persist across any type of `tracing::{x}_span!` call.

## Usage / Replication

* Run `bash run-tests.sh`
* Run `bash run-tests.sh -c` to clean directories before running
* Run `bash run-tests.sh -b <1|full>` (this just sets `RUST_BACKTRACE` env var)

## Why

This is meant to be a temporary work-around for `hyper-util`, until `tracing` bug is fixed. This is intended to track the progress of the fix.

## Related

See:

* `tracing` design defect issue: https://github.com/tokio-rs/tracing/issues/3223
* Temporary `hyper-util` PR: https://github.com/hyperium/hyper-util/pull/179
* `hyper-util` discussion: https://github.com/hyperium/hyper/issues/3870
