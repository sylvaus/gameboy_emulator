use std::fs;
use std::fs::File;
use std::io::Write;

use clap::Parser;

use crate::generator::create_instruction_function;
use crate::instruction::Instruction;
use crate::interface::Function;

mod common;
mod generator;
mod implementations;
mod instruction;
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
        file.write_all(header.to_string().as_bytes()).unwrap();
    }
    file.write_all(b"\n\n").unwrap();

    let instruction_functions: Vec<(Instruction, Function)> = instructions
        .into_iter()
        .map(|instruction| {
            let function = create_instruction_function(&instruction, &language);
            (instruction, function)
        })
        .collect();

    for (_, function) in &instruction_functions {
        file.write_all(function.definition.to_string().as_bytes())
            .unwrap();
        file.write_all(b"\n").unwrap();
    }

    let function_by_opcode = language
        .statements
        .get_function_by_opcode(&instruction_functions);
    file.write_all(function_by_opcode.definition.to_string().as_bytes())
        .unwrap();
    file.write_all(b"\n").unwrap();

    if let Some(footer) = language.statements.footer() {
        file.write_all(footer.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
