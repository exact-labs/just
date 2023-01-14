mod cli;
mod go;
mod helpers;
mod loader;
mod logger;
mod macros;
mod ops;
mod project;
mod registry;
mod runtime;
mod state;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use exact_panic::setup_panic;

#[derive(Parser)]
#[command(version = helpers::string_to_static_str(cli::get_version(false)))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup for executing external modules
    Setup,
    /// Initialize a new package.yml
    Init,
    /// Save an auth token for the registry locally
    Login,
    /// Remove the local auth token for the registry
    Logout,
    /// Verify account on registry to publish
    Verify,
    /// Package and upload this package to the registry
    Publish,
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
    /// Run a test defined in project.yml
    Test {
        #[command()]
        test: String,
    },
    /// List all tests in project.yml
    Tests,
    /// Start the index script
    Start {
        #[arg(short, long, default_value_t = String::from(""), help = "Runtime arguments")]
        args: String,
        #[arg(short = 'A', long, default_value_t = false, help = "Allow all permissions")]
        allow_all: bool,
        #[arg(long, default_value_t = false, help = "Allow environment access")]
        allow_env: bool,
        #[arg(long, default_value_t = false, help = "Allow network access")]
        allow_net: bool,
        #[arg(long, default_value_t = false, help = "Allow file system read access")]
        allow_read: bool,
        #[arg(short, long, default_value_t = false, help = "Allow file system write access")]
        allow_write: bool,
        #[arg(short, long, default_value_t = false, help = "Allow running subprocesses")]
        allow_cmd: bool,
        #[arg(short, long, default_value_t = false, help = "Allow access to system info")]
        allow_sys: bool,
    },
    /// Eval a JavaScript string
    Eval {
        #[command()]
        code: String,
        #[arg(short, long, default_value_t = String::from(""), help = "Runtime arguments")]
        args: String,
        #[arg(short = 'A', long, default_value_t = false, help = "Allow all permissions")]
        allow_all: bool,
        #[arg(long, default_value_t = false, help = "Allow environment access")]
        allow_env: bool,
        #[arg(long, default_value_t = false, help = "Allow network access")]
        allow_net: bool,
        #[arg(long, default_value_t = false, help = "Allow file system read access")]
        allow_read: bool,
        #[arg(short, long, default_value_t = false, help = "Allow file system write access")]
        allow_write: bool,
        #[arg(short, long, default_value_t = false, help = "Allow running subprocesses")]
        allow_cmd: bool,
        #[arg(short, long, default_value_t = false, help = "Allow access to system info")]
        allow_sys: bool,
    },
    /// Run a JavaScript program
    Run {
        #[arg(short, long, default_value_t = String::from(""), help = "Runtime arguments")]
        args: String,
        #[command()]
        path: String,
        #[arg(short = 'A', long, default_value_t = false, help = "Allow all permissions")]
        allow_all: bool,
        #[arg(long, default_value_t = false, help = "Allow environment access")]
        allow_env: bool,
        #[arg(long, default_value_t = false, help = "Allow network access")]
        allow_net: bool,
        #[arg(long, default_value_t = false, help = "Allow file system read access")]
        allow_read: bool,
        #[arg(short, long, default_value_t = false, help = "Allow file system write access")]
        allow_write: bool,
        #[arg(short, long, default_value_t = false, help = "Allow running subprocesses")]
        allow_cmd: bool,
        #[arg(short, long, default_value_t = false, help = "Allow access to system info")]
        allow_sys: bool,
    },
    /// Static file serving
    Serve {
        #[clap(short, long, default_value_t = String::from("0.0.0.0"))]
        host: String,
        #[clap(short, long, default_value_t = 3000)]
        port: i32,
        #[command()]
        path: String,
    },
}

fn main() {
    setup_panic!();

    let cli = Cli::parse();
    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();

    match &cli.command {
        /* essentials */
        Some(Commands::Setup) => cli::setup(),
        Some(Commands::Init) => cli::create_project_yml(),
        Some(Commands::Create) => project::create::download_template(),

        /* registry */
        Some(Commands::Login) => registry::auth::login(),
        Some(Commands::Logout) => registry::auth::logout(),
        Some(Commands::Verify) => registry::auth::verify(),
        Some(Commands::Publish) => registry::package::publish(),

        /* task runner */
        Some(Commands::Tasks) => cli::list_tasks(),
        Some(Commands::Task { task }) => cli::run_task(task),

        /* misc */
        Some(Commands::Serve { host, port, path }) => cli::serve(host.clone(), port.clone(), path),

        /* testing */
        Some(Commands::Test { test }) => cli::run_test(test),
        Some(Commands::Tests) => cli::list_tests(),

        /* package management */
        Some(Commands::Clean) => registry::manager::clean(),
        Some(Commands::Install) => registry::manager::install(),
        Some(Commands::Add { name }) => registry::manager::add(name, true),
        Some(Commands::Remove { name }) => registry::manager::remove(name),

        /* runtime */
        Some(Commands::Run {
            path,
            args,
            allow_all,
            allow_env,
            allow_net,
            allow_read,
            allow_write,
            allow_cmd,
            allow_sys,
        }) => {
            std::env::set_var("_just_args", args);
            state::Permissions::set(allow_all, allow_env, allow_net, allow_read, allow_write, allow_cmd, allow_sys);
            cli::run_exec(path, cli.verbose.is_silent(), "");
        }
        Some(Commands::Eval {
            code,
            args,
            allow_all,
            allow_env,
            allow_net,
            allow_read,
            allow_write,
            allow_cmd,
            allow_sys,
        }) => {
            std::env::set_var("_just_args", args);
            state::Permissions::set(allow_all, allow_env, allow_net, allow_read, allow_write, allow_cmd, allow_sys);
            cli::run_exec("", cli.verbose.is_silent(), code);
        }
        Some(Commands::Start {
            args,
            allow_all,
            allow_env,
            allow_net,
            allow_read,
            allow_write,
            allow_cmd,
            allow_sys,
        }) => {
            std::env::set_var("_just_args", args);
            state::Permissions::set(allow_all, allow_env, allow_net, allow_read, allow_write, allow_cmd, allow_sys);
            cli::run_exec(&project::package::read().info.index, cli.verbose.is_silent(), "");
        }

        None => cli::run_repl(),
    }
}
