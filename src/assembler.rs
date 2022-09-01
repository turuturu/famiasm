use crate::insts::{Addressing, Instruction, Opcode};
use crate::parser::Parser;
use crate::tokenizer;
use std::fs;
use std::io::{BufRead, BufReader};

pub struct Assembler {}
impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }
    pub fn assemble(&mut self, asmfilepath: String, binfilepath: String) {
        //let _ = Instruction::new(Opcode::STY, Addressing::IndirectY, Some(0x11));
        let infile = fs::File::open(asmfilepath.to_string()).unwrap();
        //        let str : String = "ss".to_string();
        //      let _ : Instruction = str.into();
        //let mut out_buf = BufWriter::new(fs::File::create(binfilepath).unwrap());
        //let reader = BufReader::new(infile);
        //for line in reader.lines() {}
        let mut v: Vec<Vec<tokenizer::Token>> = Vec::new();
        for result in BufReader::new(infile).lines() {
            if let Ok(l) = result {
                //                println!("{:?}", l);
                //let mut tokenizer = Tokenizer::new(l.to_string());
                let tokens = tokenizer::tokenize(l.to_string());
                println!("{:?}", &tokens);
                v.push(tokens);
            }
            //            println!("{:?}", l);
            //            println!("end")
        }
        let mut parser = Parser::new();
        parser.parse(v);
    }
}
