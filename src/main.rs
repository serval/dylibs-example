fn call_add(path_to_library: String, a: u32, b: u32) -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(path_to_library)?;
        let func: libloading::Symbol<unsafe extern "C" fn(a: u32, b: u32) -> u32> =
            lib.get(b"add")?;
        Ok(func(a, b))
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let Some(path_to_library) = args.get(1) else {
        eprintln!("usage: cargo run <path to dylib to load>");
        std::process::exit(1);
    };

    let a = 1;
    let b = 2;
    println!("Loading `add` function from {path_to_library:?} and asking it to add {a} and {b}");
    let sum = call_add(path_to_library.into(), a, b);
    println!("{a} + {b} = {sum:?}");
}
