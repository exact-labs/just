mod cli;
mod go;
mod helpers;
mod loader;
mod macros;
mod ops;
mod project;
mod runtime;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print version information
    #[arg(short, long)]
    version: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup for executing external modules
    Setup,
    /// Bundle module and dependencies into single file
    Bundle,
    /// Compile the script into a self contained executable
    Compile,
    /// Format source files
    Fmt,
    /// Initialize a new package.yml
    Init,
    /// Install all dependencies defined in package.yml
    Install,
    /// Manage dependencies
    Deps,
    /// Initialize a new project
    Create,
    /// Run a task defined in project.yml
    Task {
        #[command()]
        task: String,
    },
    /// List all tasks in project.yml
    Tasks,
    /// Start the index script
    Start {
        #[arg(short, long)]
        silent: bool,
    },
    /// Run a javascript program
    Run {
        #[arg(short, long)]
        silent: bool,

        #[command()]
        filename: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("{}", cli::get_version(false));
        std::process::exit(0);
    }

    match &cli.command {
        Some(Commands::Setup) => cli::setup(),
        Some(Commands::Init) => cli::create_project_yml(),
        Some(Commands::Tasks) => cli::list_tasks(),
        Some(Commands::Task { task }) => cli::run_task(task),
        Some(Commands::Create) => project::create::download_template(),
        Some(Commands::Install) => println!("install (wip)"),
        Some(Commands::Deps) => println!("deps (wip)"),
        Some(Commands::Fmt) => println!("fmt (wip)"),
        Some(Commands::Compile) => println!("compile (wip)"),
        Some(Commands::Bundle) => println!("bundle (wip)"),
        Some(Commands::Run { silent, filename }) => cli::run_exec(filename.to_string(), *silent),
        Some(Commands::Start { silent }) => cli::run_exec(project::package::read().index, *silent),
        None => cli::run_repl(),
    }
}
