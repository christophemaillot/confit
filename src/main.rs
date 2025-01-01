mod confit;
mod error;

use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use crate::confit::{perform_set, perform_insert};
use crate::error::ConfitError;

#[derive(Parser, Debug)]
struct Cli {

    /// the configuration filename to edit
    #[arg(value_name = "FILE")]
    file: PathBuf,

    /// a optional section
    #[arg(short, long, )]
    section: Option<String>,

    /// a optional extension of a backup file
    #[arg(short, long)]
    backup: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

    /// Set a value
    Set {
        /// The key of the value to set
        #[arg()]
        key: String,

        /// The  value to set
        #[arg()]
        value: String,

        /// The separator
        #[arg(short, long, default_value = "=")]
        separator: String,
    },

    /// Insert a line after the given tag
    Insert {

        /// the marker line (see documentation)
        #[arg()]
        marker: String,

        /// the line to insert
        #[arg()]
        line: String,

    },

}

fn main() -> Result<(), ConfitError> {

    let args = Cli::parse();

    // check file exists
    if !Path::exists(&args.file) {
        println!("ERROR : file not found");
        std::process::exit(1);
    }

    let result = match args.command {
        Commands::Set {key, value, separator } => {
            perform_set(&args.file, &key, &value, &separator)
        }
        Commands::Insert { line, marker } => {
            perform_insert(&args.file, &line, &marker)
        }
    };


    if let Err(e) = result {
        println!("ERROR : {}", e);
    }

    Ok(())
}
