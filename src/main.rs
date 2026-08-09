use clap::{Parser, Subcommand};
use owo_colors::set_override;
use std::env;
use whichway::doctor::run_doctor;
use whichway::report::{doctor_explain, explain};
use whichway::resolvers::resolve_all;

#[derive(Parser, Debug)]
#[command(
    name = "whichway",
    version,
    about = "Explain why a shell command resolves to a specific binary",
    long_about = "Read-only CLI that inspects PATH and explains command resolution.\n\
                  Detects shadowed duplicates, symlinks, shims, and version-manager conflicts \
                  (asdf, nvm, pyenv). Diagnoses only — never modifies your system.",
    after_long_help = "EXAMPLES:\n  \
        whichway python          Show every python match in PATH, in resolution order\n  \
        whichway doctor          Scan PATH for duplicates, broken symlinks, orphan shims\n  \
        whichway python --json   Same as above, but output JSON for scripting"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Output results as JSON instead of human-readable text.
    #[arg(long, global = true)]
    json: bool,

    /// Disable ANSI color in terminal output.
    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Scan PATH for duplicates, broken symlinks, and orphaned shims.
    ///
    /// Walks every directory in PATH and reports command names that appear more
    /// than once, symlinks whose target is missing, and shim scripts left behind
    /// by version managers that are no longer installed.
    Doctor,

    /// Show how a command name resolves through PATH (default when a name is given).
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
                eprintln!("Error: couldn't determine home directory, cannot run doctor");
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
