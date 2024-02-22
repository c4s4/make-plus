use clap::Parser;
use make_plus::{self, HelpLine};

/// Print help on makefile targets
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Parse root makefile only
    #[clap(short, long)]
    root: bool,
    /// Target to get description for
    target: String,
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
    let mut help_lines = make_plus::parse_makefile(makefile, !args.root);
    // print target description
    print_target_desc(&mut help_lines, &args.target);
}

/// Print description for target
fn print_target_desc(help_lines: &mut Vec<HelpLine>, target: &str) {
    for line in help_lines.iter() {
        if line.name == target {
            println!("{}", line.description);
            return;
        }
    }
    eprintln!("target {} not found", target);
    std::process::exit(1);
}
