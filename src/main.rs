use clap::{Parser, Subcommand};
use owo_colors::set_override;
use std::env;
use whichway::doctor::run_doctor;
use whichway::report::{doctor_explain, explain};
use whichway::resolvers::resolve_all;

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
        Command::Doctor => {
            let path_var = env::var("PATH").unwrap_or_default();
            let Some(home) = dirs::home_dir() else {
                eprintln!("Warning: couldn't determine home directory, skipping orphan shim check");
                std::process::exit(1);
            };

            let report = run_doctor(&path_var, &home);

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
                return;
            }

            println!("{}", doctor_explain(&report));
        }
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
