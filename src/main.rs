use clap::Parser;
use std::env;
use whichway::report::explain;
use whichway::resolvers::resolve_all;
use owo_colors::set_override;

#[derive(Parser, Debug)]
#[command(name = "whichway")]
struct Cli {
    command: String,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    no_color: bool,
}

fn main() {
    let cli = Cli::parse();
    let path_var = env::var("PATH").unwrap_or_default();
    let results = resolve_all(&cli.command, &path_var);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return;
    }

    if cli.no_color {
        set_override(false);
    }

    if results.is_empty() {
        println!("No matches found for: {}", cli.command);
        return;
    }

    println!("Resolution order for: {}", cli.command);
    for (i, m) in results.iter().enumerate() {
        println!("  {}. {}  {}", i + 1, m.path.display(), explain(m));
    }
}
