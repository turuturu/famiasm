use crate::parser::Parser;
use crate::tokenizer;
use log::debug;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub struct Assembler {}
impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }
    pub fn assemble(&mut self, asmfilepath: String, binfilepath: String) {
        let infile = fs::File::open(asmfilepath.to_string()).unwrap();
        let mut v: Vec<Vec<tokenizer::Token>> = Vec::new();
        for result in BufReader::new(infile).lines() {
            if let Ok(l) = result {
                let tokens = tokenizer::tokenize(l.to_string());
                debug!("{:?}", &tokens);
                v.push(tokens);
            }
        }
        let mut parser = Parser::new();
        parser.set_base_path(&asmfilepath);
        parser.parse(v);
        let bin = parser.gen_binary();
        let mut file = File::create(binfilepath).unwrap();
        file.write_all(&bin).unwrap();
    }
}
