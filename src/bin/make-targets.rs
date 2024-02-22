use clap::Parser;
use make_plus::{self, HelpLine};

/// Print list of targets
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Parse root makefile only
    #[clap(short, long)]
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
        }
    };
    // parse makefile
    let mut help_lines = make_plus::parse_makefile(makefile, !args.root);
    // print target description
    print_target_list(&mut help_lines);
}

/// Print list of targets
fn print_target_list(help_lines: &mut Vec<HelpLine>) {
    let list = help_lines
        .iter()
        .map(|line| line.name.clone())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", list);
}
