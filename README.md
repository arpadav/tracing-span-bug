# `tracing` bug with spans

For projects which have:

* a default logger using `tracing`
* spawn multiple threads
* each thread has its own async runtime
* each thread has its own `tracing` logger span

Then, performing a `reqwest::Client::get` will cause a panic within any of the async runtimes. Why? Not sure. This is a problem with `tracing`

However, problem is fixed indirectly by removing any `tracing` spans created/entered in the `hyper-util` crate (dependency of `reqwest`).

## Why

This is meant to be a temporary work-around, until `tracing` bug is fixed. This is intended to track the progress of the fix.

## Usage

Run `bash run-tests.sh`

## Related

See:

* https://github.com/hyperium/hyper-util/pull/134#issuecomment-2773237311
* Temporary PR: https://github.com/hyperium/hyper-util/pull/179
* Discussion: https://github.com/hyperium/hyper/issues/3870
