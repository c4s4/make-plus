use clap::Parser;
use make_plus::{self, HelpLine};

/// Run command ensuring only one instance is running on this system
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Parse root makefile only
    #[clap(short, long, default_value_t = false)]
    root: bool,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // find makefile
    let makefile = match make_plus::find_makefile() {
        Some(makefile) => makefile,
        None => {
            eprintln!("makefile not found");
            std::process::exit(1);
        },
    };
    // parse makefile
    let mut help_lines = make_plus::parse_makefile(makefile, args.root);
    // print help
    print_help(&mut help_lines);
}

/// Print help lines
fn print_help(help_lines: &mut Vec<HelpLine>) {
    // get max length of name
    let max_name_len = help_lines.iter().map(|line| line.name.len()).max().unwrap();
    // sort help lines by name
    help_lines.sort_by(|a, b| a.name.cmp(&b.name));
    // iterate over help lines
    for line in help_lines {
        // print name
        print!("\x1b[93m{:width$}\x1b[0m", line.name, width = max_name_len);
        // print description
        print!(" {}", line.description);
        // print dependencies
        if line.dependencies.len() > 0 {
            print!(" [{}]", line.dependencies.join(" "));
        }
        // print newline
        println!();
    }
}
