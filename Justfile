all:
    just --list

# Check that the crate will compile for wasm.
check-wasm:
    cargo check --target wasm32-unknown-unknown