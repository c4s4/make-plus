use std::path::Path;
use regex::Regex;

/// FILE_NAMES is a list of possible makefile names
const FILE_NAMES: [&str; 3] = ["makefile", "Makefile", "GNUmakefile"];

/// find_makefile returns name of found makefile
pub fn find_makefile() -> Option<String> {
    for name in FILE_NAMES {
        if Path::new(name).exists() {
            return Some(name.to_string());
        }
    }
    None
}

/// HelpLine is a struct that represents a line in a makefile
#[derive(Debug)]
pub struct HelpLine {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

/// parse_makefile returns HelpLines
pub fn parse_makefile(file: String, recursive: bool) -> Vec<HelpLine> {
    let contents = match std::fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("ERROR reading makefile: {}", err);
            std::process::exit(1);
        }
    };
    let mut help_lines: Vec<HelpLine>  = vec![];
    let re = Regex::new(r"(?m)^([\w-]+):[\t ]*([^#\n]+)[\t ]*(#[\t ]*(.*))$").unwrap();
    for (_, [name, deps, _, description]) in re.captures_iter(contents.as_str()).map(|c| c.extract()) {
        let dependencies: Vec<String> = deps.split_whitespace().map(|s| s.to_string()).collect();
        let help_line = HelpLine {
            name: name.to_string(),
            description: description.to_string(),
            dependencies: dependencies,
        };
        help_lines.push(help_line);
    };
    help_lines
}
