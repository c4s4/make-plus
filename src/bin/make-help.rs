use clap::Parser;
use make_plus::{self, HelpLine};

/// Print help on makefile targets
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Makefile to parse
    #[clap(short, long)]
    file: Option<String>,
    /// Parse root makefile only
    #[clap(short, long)]
    root: bool,
    /// Don't print targets without description
    #[clap(short, long)]
    mute: bool,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // find makefile
    let makefile = match args.file {
        Some(file) => file,
        None => match make_plus::find_makefile() {
            Some(makefile) => makefile,
            None => {
                eprintln!("makefile not found");
                std::process::exit(1);
            }
        },
    };
    // parse makefile
    let mut help_lines = make_plus::parse_makefile(makefile, !args.root);
    // print help
    print_help(&mut help_lines, args.mute);
}

/// Print help lines
fn print_help(help_lines: &mut Vec<HelpLine>, mute: bool) {
    // get max length of name
    let max_name_len = help_lines.iter().map(|line| line.name.len()).max().unwrap();
    // iterate over help lines
    for line in help_lines {
        // skip if mute and no description
        if mute && line.description.len() == 0 {
            continue;
        }
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
