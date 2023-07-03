use std::fs;
use std::fs::File;
use std::io::Write;

use clap::Parser;

use crate::generator::create_instruction_function;

mod generator;
mod implementations;
mod interface;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "generator/data/instructions.json")]
    data_path: String,

    #[arg(short, long, default_value = "emulator/src/generated/instructions.rs")]
    gen_path: String,
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(&args.data_path).unwrap_or_else(|error| {
        panic!("Could not read file: {}, error {:?}", args.data_path, error)
    });

    let instructions = parser::parse_instructions(&content).expect("Could not parse instructions");

    let language = implementations::rust::get_rust_language();

    let mut file = File::create(&args.gen_path).unwrap_or_else(|error| {
        panic!(
            "Could not open file {} in write mode: {}",
            args.gen_path, error
        )
    });

    if let Some(header) = language.statements.header() {
        file.write(header.to_string().as_bytes()).unwrap();
    }
    file.write(b"\n").unwrap();

    for instruction in &instructions {
        if let Some(function) = create_instruction_function(instruction, &language) {
            file.write(function.definition.to_string().as_bytes())
                .unwrap();
            file.write(b"\n").unwrap();
        }
    }
    if let Some(footer) = language.statements.footer() {
        file.write(footer.to_string().as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}