echo "Cleaning..."
echo "------------------------------------------------------------------------"
cd hyper-util-dns-with-span; cargo clean -q; cd - > /dev/null
cd hyper-util-dns-without-span; cargo clean -q; cd - > /dev/null
file="hyper-util-dns-with-span/src/lib.rs"
hash=$(tr -d '\r' < "$file" | sha256sum | cut -c1-32)
echo "Hash of $file"
echo "    $hash"
file="hyper-util-dns-without-span/src/lib.rs"
hash=$(tr -d '\r' < "$file" | sha256sum | cut -c1-32)
echo "Hash of $file"
echo "    $hash"
echo "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
echo "Files are identical. See respective Cargo.toml for different \`hyper-util\` versions."
echo "------------------------------------------------------------------------"
echo "Running tests: *with* tracing span in hyper-util DNS name-resolution. *This should fail if tracing bug is present.* If passes, then the bug is fixed."
echo "------------------------------------------------------------------------"
echo "Building (quietly)..."
echo "------------------------------------------------------------------------"
cd hyper-util-dns-with-span
cargo build --quiet
RUST_BACKTRACE=FULL cargo test --package hyper-util-dns-with-span --lib -- test --exact --show-output
cd -
echo
echo "------------------------------------------------------------------------"
echo "Running tests: tracing span *removed* from hyper-util DNS name-resolution"
echo "------------------------------------------------------------------------"
echo "Building (quietly)..."
echo "------------------------------------------------------------------------"
cd hyper-util-dns-without-span
cargo build --quiet
RUST_BACKTRACE=FULL cargo test --package hyper-util-dns-without-span --lib -- test --exact --show-output
cd -