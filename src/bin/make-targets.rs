use clap::Parser;
use make_plus::{self, HelpLine};

/// Print list of targets
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
    // print target description
    print_target_list(&mut help_lines, args.mute);
}

/// Print list of targets
fn print_target_list(help_lines: &mut Vec<HelpLine>, mute: bool) {
    let mut list = vec![];
    for line in help_lines.iter() {
        if mute && line.description.len() == 0 {
            continue;
        }
        list.push(line.name.clone());
    }
    println!("{}", list.join(" "));
}
