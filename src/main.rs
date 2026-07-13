use clap::Parser;
use std::env;
use whichway::report::explain;
use whichway::resolvers::resolve_all;

#[derive(Parser, Debug)]
#[command(name = "whichway")]
struct Cli {
    command: String,
}

fn main() {
    let cli = Cli::parse();
    let path_var = env::var("PATH").unwrap_or_default();
    let results = resolve_all(&cli.command, &path_var);

    if results.is_empty() {
        println!("No matches found for: {}", cli.command);
        return;
    }

    println!("Resolution order for: {}", cli.command);
    for (i, m) in results.iter().enumerate() {
        println!("  {}. {}  {}", i, m.path.display(), explain(m));
    }
}
