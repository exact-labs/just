mod cli;
mod helpers;
mod loader;
mod ops;
mod project;
mod registry;
mod runtime;

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
enum Cache {
    /// Remove the Just package cache
    Clean,
}

#[derive(Subcommand)]
enum Registry {
    /// Set the default registry
    Set {
        #[command()]
        url: String,
    },
    /// Reset to standard registry [r.justjs.dev]
    Clear,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup for executing external modules
    Setup,
    /// Initialize a new package.yml
    Init,
    /// Save an auth token for the registry locally
    Login {
        #[arg(short, long, default_value_t = registry::get_default(), help = "Package registry url")]
        registry: String,
    },
    /// Remove the local auth token for the registry
    Logout,
    /// Verify account on registry to publish
    Verify {
        #[arg(short, long, default_value_t = registry::get_default(), help = "Package registry url")]
        registry: String,
    },
    /// Package and upload this package to the registry
    Publish {
        #[arg(short, long, default_value_t = registry::get_default(), help = "Package registry url")]
        registry: String,
    },
    /// Install all dependencies defined in package.yml
    Install {
        #[arg(short, long, default_value_t = registry::get_default(), help = "Package registry url")]
        registry: String,
    },
    /// Add a new dependency
    Add {
        #[command()]
        name: String,
        #[arg(short, long, default_value_t = registry::get_default(), help = "Package registry url")]
        registry: String,
    },
    /// Remove a dependency
    Remove {
        #[command()]
        name: String,
    },
    /// Remove unused dependencies
    Clean,
    /// Manage the Just package cache
    Cache {
        #[command(subcommand)]
        command: Cache,
    },
    /// Manage registry settings
    Registry {
        #[command(subcommand)]
        command: Registry,
    },
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

    if registry::get_default() == "" {
        registry::set_default(&String::from("https://r.justjs.dev"), true)
    }

    let cli = Cli::parse();
    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();

    match &cli.command {
        /* essentials */
        Some(Commands::Setup) => cli::setup(),
        Some(Commands::Init) => cli::create_project_yml(),
        Some(Commands::Create) => project::create::download_template(),

        /* registry */
        Some(Commands::Login { registry }) => registry::auth::login(registry),
        Some(Commands::Logout) => registry::auth::logout(),
        Some(Commands::Verify { registry }) => registry::auth::verify(registry),
        Some(Commands::Publish { registry }) => registry::package::publish(registry),

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
        Some(Commands::Install { registry }) => registry::manager::install(registry),
        Some(Commands::Add { name, registry }) => registry::manager::add(name, true, registry),
        Some(Commands::Remove { name }) => registry::manager::remove(name),

        /* cache management */
        Some(Commands::Cache { command }) => match command {
            Cache::Clean => cli::cache_clean(),
        },

        /* registry management */
        Some(Commands::Registry { command }) => match command {
            Registry::Set { url } => registry::set_default(url, false),
            Registry::Clear => registry::set_default(&String::from("https://r.justjs.dev"), false),
        },

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
            state::permissions::set(allow_all, allow_env, allow_net, allow_read, allow_write, allow_cmd, allow_sys);
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
            state::permissions::set(allow_all, allow_env, allow_net, allow_read, allow_write, allow_cmd, allow_sys);
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
            state::permissions::set(allow_all, allow_env, allow_net, allow_read, allow_write, allow_cmd, allow_sys);
            cli::run_exec(&project::package::read().info.index, cli.verbose.is_silent(), "");
        }

        None => cli::run_repl(),
    }
}
