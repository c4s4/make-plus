use anyhow::Result;
use clap::Parser;
use make_plus::{self, HelpLine};

/// Describe given target
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Makefile to parse
    #[clap(short, long)]
    file: Option<String>,
    /// Parse root makefile only
    #[clap(short, long)]
    root: bool,
    /// Target to get description for
    target: String,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    match run(args) {
        Ok(_) => println!("OK"),
        Err(e) => eprintln!("ERROR: {:#}", e),
    }
}

fn run(args: Cli) -> Result<()> {
    // find makefile
    let makefile = match args.file {
        Some(file) => file,
        None => match make_plus::find_makefile() {
            Some(makefile) => makefile,
            None => anyhow::bail!("makefile not found"),
        },
    };
    // parse makefile
    let mut help_lines = make_plus::parse_makefile(makefile, !args.root)?;
    // print target description
    print_target_desc(&mut help_lines, &args.target);
    Ok(())
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
