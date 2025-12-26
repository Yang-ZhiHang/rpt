set shell := ["cmd", "/c"]

run example:
    cargo run --example {{example}}

# Run example with release build and native CPU flags (enables SIMD on your CPU)
run-fast example:
    set RUSTFLAGS=-C target-cpu=native && cargo run --release --example {{example}}