use anyhow::Result;
use clap::{Parser, Subcommand};

mod bridge;

#[derive(Parser)]
#[command(name = "polyscript", about = "Run Python/Julia/C++ scripts via FFI bridges")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Python script
    Py {
        /// Path to .py script
        script: String,
        /// Arguments passed to the script
        args: Vec<String>,
    },
    /// Run a Julia script
    Jl {
        /// Path to .jl script
        script: String,
        args: Vec<String>,
    },
    /// Run a C++ shared library function
    Cpp {
        /// Path to compiled .so/.dylib
        lib: String,
        /// Function name to call
        func: String,
        args: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Py { script, args } => bridge::python::run(&script, &args),
        Commands::Jl { script, args } => bridge::julia::run(&script, &args),
        Commands::Cpp { lib, func, args } => bridge::cpp::run(&lib, &func, &args),
    }
}
