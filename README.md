# `tracing` bug with spans

For projects which have:

* a default logger using `tracing`
* spawn multiple threads
* each thread has its own async runtime
* each thread has its own `tracing` logger span

Then, performing a `reqwest::Client::get` will cause a panic within any of the async runtimes. Why? Not sure. This is a problem with `tracing`

However, problem is fixed indirectly by removing any `tracing` spans created/entered in the `hyper-util` crate (dependency of `reqwest`).

## Usage / Replication

* Run `bash run-tests.sh`
* Run `bash run-tests.sh -c` to clean directories before running
* Run `bash run-tests.sh -b <1|full>` to show backtrace (this just sets `RUST_BACKTRACE` env var)

## Why

This is meant to be a temporary work-around for `hyper-util`, until `tracing` bug is fixed. This is intended to track the progress of the fix.

## Related

See:

* https://github.com/hyperium/hyper-util/pull/134#issuecomment-2773237311
* Temporary `hyper-util` PR: https://github.com/hyperium/hyper-util/pull/179
* `hyper-util` Discussion: https://github.com/hyperium/hyper/issues/3870
