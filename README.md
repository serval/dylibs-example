# dylibs-example

This is a small example repository showing how to write a dynamic library with Rust and how to dynamically load that library from disk in an example program.

## Details

The [example library](./src/lib.rs) exports a function `add(u32, u32) -> u32)` that adds two numbers.

The [demo app](./src/main.rs) accepts the path to a dylib file (`.dylib` on macOS, `.so` on Linux), loads it into memory via [libloading](https://crates.io/crates/libloading), and invokes the `add` function.

## Usage

This project uses [just](https://github.com/casey/just) (`brew install just`) for development workflows and automation. Run `just` with no arguments to see a list of available commands.

Build the library:

```shell
just build
```

Run the example program (which uses the library):

```shell
just run
```

If all goes well, yous hould see something like this:

```
Loading `add` function from "target/debug/libdylibs_example.dylib" and asking it to add 1 and 2
1 + 2 = Ok(3)
```
