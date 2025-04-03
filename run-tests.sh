#!/bin/bash
# Run tests for: https://github.com/arpadav/tracing-span-bug
# --------------------------------------------------
# parse cli args
# --------------------------------------------------
CLEAN=0
BACKTRACE=0
while [[ $# -gt 0 ]]; do
    case "$1" in
    -c | --clean)
        CLEAN=1
        shift
        ;;
    -b | --backtrace)
        BACKTRACE=$2
        if [ "$BACKTRACE" != "1" ] && [ "$BACKTRACE" != "full" ]; then
            echo "Unknown argument for -b|--backtrace: $2"
            echo "Usage: bash run-tests.sh [-c|--clean] [-b|--backtrace <1|full>]"
            exit 1
        fi
        shift 2
        ;;
    *)
        echo "Unknown argument: $1"
        echo "Usage: bash run-tests.sh [-c|--clean] [-b|--backtrace <1|full>]"
        exit 1
        ;;
    esac
done
# --------------------------------------------------
# clean
# --------------------------------------------------
if [ "$CLEAN" = "1" ]; then
    echo "Cleaning..."
    echo "------------------------------------------------------------------------"
    cd hyper-util-dns-with-span; cargo clean -q; cd - > /dev/null
    cd hyper-util-dns-without-span; cargo clean -q; cd - > /dev/null
fi
# --------------------------------------------------
# set RUST_BACKTRACE
# --------------------------------------------------
if [ "$BACKTRACE" = "0" ]; then
    unset BACKTRACE
else
    export RUST_BACKTRACE="$BACKTRACE"
fi
# --------------------------------------------------
# get file hashes, ensure they are identical. only difference
# should be in their Cargo.toml, patching `hyper-util` as 
# different versions
# --------------------------------------------------
file="hyper-util-dns-with-span/src/lib.rs"
hash0=$(tr -d '\r' < "$file" | sha256sum | cut -c1-32)
echo "Hash of $file"
echo "    $hash0"
file="hyper-util-dns-without-span/src/lib.rs"
hash1=$(tr -d '\r' < "$file" | sha256sum | cut -c1-32)
echo "Hash of $file"
echo "    $hash1"
if [ "$hash0" != "$hash1" ]; then
    echo "Files are different - they should be identical. Aborting."
    exit 1
fi
echo "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
echo "Files are identical. See respective Cargo.toml for different \`hyper-util\` versions."
# --------------------------------------------------
# run tests
# --------------------------------------------------
echo "------------------------------------------------------------------------"
echo "Running tests: tracing span *removed* from hyper-util DNS name-resolution". These should pass.
echo "------------------------------------------------------------------------"
echo "Building (quietly)..."
echo "------------------------------------------------------------------------"
cd hyper-util-dns-without-span
cargo build --quiet
cargo test --package hyper-util-dns-without-span --lib -- --show-output 2>&1 | tee ../tests-without-span.output
cd - > /dev/null
if [ "$CLEAN" = "0" ]; then
    sleep 3
fi
echo "------------------------------------------------------------------------"
echo "Running tests: *with* tracing span in hyper-util DNS name-resolution. *This should fail if tracing bug is present.* If passes, then the bug is fixed."
echo "------------------------------------------------------------------------"
echo "Building (quietly)..."
echo "------------------------------------------------------------------------"
cd hyper-util-dns-with-span
cargo build --quiet
cargo test --package hyper-util-dns-with-span --lib -- --show-output 2>&1 | tee ../tests-with-span.output
cd - > /dev/null
echo "------------------------------------------------------------------------"
echo "Logs saved to:"
echo "    tests-with-span.output"
echo "    tests-without-span.output"