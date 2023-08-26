use std::fs;

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
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(&args.data_path)
        .unwrap_or_else(|error| panic!("Could not read file: {}, error {:?}", args.data_path, error));

    let instructions = parser::parse_instructions(&content).expect("Could not parse instructions");

    let language = implementations::rust::get_rust_language();

    for instruction in &instructions {
        if let Some(function) = create_instruction_function(instruction, &language) {
            println!("{}", function.definition.to_string())
        }
    }
}
