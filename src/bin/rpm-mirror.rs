use std::{fs::File, path::PathBuf, process::ExitCode};

use clap::Parser;
use xml::reader::{ParserConfig, XmlEvent};

#[derive(Debug, Parser)]
struct CommandLineArguments {
    #[arg(value_name = "FILE")]
    filename: PathBuf,
}

fn main() -> ExitCode {
    let cli_arguments = CommandLineArguments::parse();
    let config = ParserConfig::new()
        .trim_whitespace(true)
        .cdata_to_characters(false);

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

    let reader = config.create_reader(reader);

    for event in reader {
        match event {
            Ok(item) => {
                handle_xml_event(item);
            }
            Err(_) => {}
        }
    }

    return ExitCode::SUCCESS;
}

fn handle_xml_event(item: XmlEvent) {
    match item {
        XmlEvent::StartDocument { .. } => { /* We don't care about this */ }
        XmlEvent::EndDocument => { /* We don't care about this */ }
        XmlEvent::ProcessingInstruction { .. } => { /* We don't care about this */ }
        XmlEvent::StartElement { name, .. } => {
            println!("Starting to process: {}", name.local_name);
        }
        XmlEvent::EndElement { name } => {
            println!("Done processing: {}", name.local_name);
        }
        XmlEvent::CData(string) => {
            println!("Read {string}");
        }
        XmlEvent::Comment(_) => { /* We don't care about this */ }
        XmlEvent::Characters(string) => {
            println!("Read {string}");
        }
        XmlEvent::Whitespace(string) => {
            println!("Read {string}");
        }
    }
}
