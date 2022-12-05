mod cli;
mod go;
mod helpers;
mod loader;
mod logger;
mod macros;
mod ops;
mod platform;
mod project;
mod runtime;

use clap::{Parser, Subcommand};
use platform::{compile::CompileOptions, CommandRunner};

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
    /// Build the script into a self contained executable
    Build,
    /// Run the transformer
    Compile(Box<CompileOptions>),
    /// Format source files
    Fmt,
    /// Initialize a new package.yml
    Init,
    /// Install all dependencies defined in package.yml
    Install,
    /// Add a new dependency
    Add {
        #[command()]
        name: String,
    },
    /// Remove a dependency
    Remove {
        #[command()]
        name: String,
    },
    /// Remove unused dependencies
    Clean,
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
    /// Run a JavaScript or TypeScript program
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
        Some(Commands::Install) => cli::DependencyManager::install(),
        Some(Commands::Add { name }) => cli::DependencyManager::add(name),
        Some(Commands::Remove { name }) => cli::DependencyManager::remove(name),
        Some(Commands::Clean) => cli::DependencyManager::clean(),
        Some(Commands::Compile(options)) => options.execute(),
        Some(Commands::Fmt) => println!("fmt (wip)"),
        Some(Commands::Build) => println!("build (wip)"),
        Some(Commands::Bundle) => println!("bundle (wip)"),
        Some(Commands::Run { silent, filename }) => cli::run_exec(filename.to_string(), *silent),
        Some(Commands::Start { silent }) => cli::run_exec(project::package::read().index, *silent),
        None => cli::run_repl(),
    }
}
