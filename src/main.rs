use whichway::{find_matches};
use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
struct Cli {
    command: String,
}

fn main() {
    let cli = Cli::parse();
    let path_var = env::var("PATH").unwrap_or_default();
    let matches = find_matches(&cli.command, &path_var);

    if matches.is_empty() {
        println!("No matches found for: {}", cli.command);
    } else {
        println!("Resolution order for: {}", cli.command);
        for (i, path) in matches.iter().enumerate() {
            println!("  {}. {}", i, path.display());
        }
    }
}