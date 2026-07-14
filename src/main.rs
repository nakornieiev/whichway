use clap::{Parser, Subcommand};
use std::env;
use whichway::report::explain;
use whichway::resolvers::resolve_all;
use owo_colors::set_override;

#[derive(Parser, Debug)]
#[command(version, about, name = "whichway")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(long, global = true)]
    json: bool,

    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    Doctor,

    #[command(external_subcommand)]
    Resolve(Vec<String>),
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        set_override(false);
    }

    match &cli.command {
        Command::Doctor => { todo!(); }
        Command::Resolve(args) => {
            let Some(arg) = args.first() else {
                eprintln!("Usage: whichway <command> | whichway doctor");
                std::process::exit(1);
            };

            let path_var = env::var("PATH").unwrap_or_default();
            let results = resolve_all(arg, &path_var);

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&results).unwrap());
                return;
            }

            if results.is_empty() {
                println!("No matches found for: {}", arg);
                return;
            }

            println!("Resolution order for: {}", arg);
            for (i, m) in results.iter().enumerate() {
                println!("  {}. {}   {}", i + 1, m.path.display(), explain(m));
            }
        }
    }
}
