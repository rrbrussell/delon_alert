use std::{fs::File, path::PathBuf, process::ExitCode};

use clap::Parser;

use delon_alert::*;

#[derive(Debug, Parser)]
struct CommandLineArguments {
    #[arg(value_name = "FILE")]
    filename: PathBuf,
}

fn main() -> ExitCode {
    let cli_arguments = CommandLineArguments::parse();

    println!("{cli_arguments:#?}");
    print!("\n");

    let Ok(filename) = cli_arguments.filename.canonicalize() else {
        println!("Unable to canonicalize filename.");
        return ExitCode::FAILURE;
    };

    let Ok(reader) = File::open(&filename) else {
        println!("Unable to open {filename:#?}");
        return ExitCode::FAILURE;
    };

    match yaserde::de::from_reader::<File, Repomd>(reader) {
        Err(e) => {
            println!("Failed to read input data");
            println!("{e}");
            return ExitCode::FAILURE;
        }
        Ok(repomd) => {
            println!("{repomd:#?}");
        }
    };

    return ExitCode::SUCCESS;
}
