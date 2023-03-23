# List available recipes
help:
    just -l

# Build the dylib
build:
    cargo build --lib

# Run the demo app with the given dylib (defaults to the output of `just build`, assuming macOS)
run DYLIB='target/debug/libdylibs_example.dylib':
    cargo run -- {{DYLIB}}