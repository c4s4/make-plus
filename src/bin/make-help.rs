use clap::Parser;
use std::path::Path;
// use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const FILE_NAMES: [&str; 3] = ["makefile", "Makefile", "GNUmakefile"];

/// Run command ensuring only one instance is running on this system
#[derive(Parser)]
struct Cli {
    /// The lone version
    #[arg(short, long)]
    version: bool,
    /// Software to set version for
    #[arg(default_value(""))]
    software: String,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // print version and exit
    if args.version {
        println!("{}", VERSION);
        return;
    }
    // find makefile
    let makefile = find_makefile().unwrap();
    // unwrap_or(error("makefile not found"));
    println!("makefile: {makefile}");
}

/// find_makefile returns name of found makefile
fn find_makefile() -> Option<String> {
    for name in FILE_NAMES {
        if Path::new(name).exists() {
            return Some(name.to_string());
        }
    }
    None
}

// // error prints error message and exits with error code 1
// fn error(msg: &str) {
//     eprintln!("ERROR {msg}");
//     process::exit(1);
// }