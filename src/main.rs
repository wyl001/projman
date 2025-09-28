use projman::{scan_projects, start_project_by_yml, Cli, Commands};
use std::error::Error;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("错误: {}", e);
        std::process::exit(1)
    }
}


fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Commands::List { config } => {
            scan_projects(config)?;
        }
        Commands::Start { name, config } => {
            start_project_by_yml(&name, &config)?;
        }
    }
    Ok(())
}
