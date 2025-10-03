use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

/// The command to run for creating a project
#[derive(Parser)]
struct Command {
    /// The name of the project folder
    name: String,

    /// The path to create the project in (defaults to current working directory)
    path: Option<std::path::PathBuf>,

    /// Print debugging info while running command
    #[clap(long, short, action)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Command::parse();

    if args.debug {
        println!();
        printinfo("Running in debug mode.");
        println!()
    }

    let name = &args.name;

    let path = match args.path {
        Some(path) => path,
        None => std::env::current_dir()?,
    };

    if args.debug {
        println!("{} {}", "Project name:".green().bold(), name);
        println!("{} {}", "Project path:".green().bold(), path.display())
    }

    let project_path = format!("{}/{}", path.display(), name);

    std::fs::create_dir(&project_path)
        .with_context(|| format!("Unable to create project directory at '{}'", &project_path))?;

    Ok(())
}

fn printinfo(message: &str) {
    println!("{}", message.yellow().bold())
}
