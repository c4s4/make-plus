use anyhow::{Context, Result};
use regex::Regex;
use shellexpand;
use std::path::Path;

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

/// HELP_LINE_RE is a regex to match a line in a makefile
const HELP_LINE_RE: &str = r"(?m)^([\w-]+):[\t ]*([^#\n]*)[\t ]*(([#\t ]*)(.*))";

/// HelpLine is a struct that represents a line in a makefile
#[derive(Debug)]
pub struct HelpLine {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

/// parse_makefile returns HelpLines
pub fn parse_makefile(file: String, recursive: bool) -> Result<Vec<HelpLine>> {
    let contents =
        std::fs::read_to_string(&file).with_context(|| format!("reading makefile: {}", file))?;
    let mut help_lines: Vec<HelpLine> = vec![];
    let re = Regex::new(HELP_LINE_RE).unwrap();
    for (_, [name, deps, _, _, description]) in
        re.captures_iter(contents.as_str()).map(|c| c.extract())
    {
        let dependencies: Vec<String> = deps
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect();
        let help_line = HelpLine {
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            dependencies: dependencies,
        };
        help_lines.push(help_line);
    }
    if recursive {
        let filenames = find_included_files(&contents);
        for filename in filenames {
            // expand user home directory
            let file = shellexpand::tilde(&filename).to_string();
            let mut included = parse_makefile(file, true)?;
            help_lines.append(&mut included);
        }
    }
    // sort help lines by name
    help_lines.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(help_lines)
}

/// return included makefiles
fn find_included_files(contents: &str) -> Vec<String> {
    let mut included = vec![];
    let re = Regex::new(r"(?m)^-?include\s+(.*)$").unwrap();
    for (_, [filename]) in re.captures_iter(contents).map(|c| c.extract()) {
        included.push(filename.to_string());
    }
    included
}
