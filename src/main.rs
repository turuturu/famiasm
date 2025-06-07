use log::debug;
use std::env;
mod assembler;
use assembler::Assembler;
mod common;
mod directive;
mod insts;
mod nes_header;
mod parser;
mod symbol_table;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: famiasm <filename.asm>");
        return;
    }
    debug!("{:?}", args);
    let filename = &args[1];
    let output_filename = filename.replace(".asm", ".nes");

    let mut assembler = Assembler::new();
    assembler.assemble(filename.to_string(), output_filename);
}
