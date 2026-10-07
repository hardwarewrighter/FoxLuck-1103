fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => {
            println!("Hello, Luckfox 1103!");
            println!("Rust CLI running on {} Linux", std::env::consts::ARCH);
        }
        [flag] if flag == "--help" || flag == "-h" => {
            println!("Usage: hello-luckfox-1103 [--help | --version]");
            println!("A Rust CLI for the Luckfox Pico RV1103.");
        }
        [flag] if flag == "--version" || flag == "-V" => {
            println!("hello-luckfox-1103 {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Usage: hello-luckfox-1103 [--help | --version]");
            std::process::exit(2);
        }
    }
}
