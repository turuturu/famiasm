use std::fs;
use std::io::{BufRead, BufReader};
use crate::insts::{Opcode, Addressing, Instruction};
use crate::tokenizer::{Tokenizer};
//use crate::insts;
/*use crate::insts::Opcode;
use crate::insts::Addressing;
use crate::insts::Instruction;
*/
pub struct Assembler {}
impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }
    pub fn assemble(&mut self, asmfilepath: String, binfilepath: String) {
        let _ = Instruction::new(Opcode::STY, Addressing::IndirectY, Some(0x11));
        let infile = fs::File::open(asmfilepath.to_string()).unwrap();
//        let str : String = "ss".to_string();
  //      let _ : Instruction = str.into();
        //let mut out_buf = BufWriter::new(fs::File::create(binfilepath).unwrap());
        //let reader = BufReader::new(infile);
        //for line in reader.lines() {}
        for result in BufReader::new(infile).lines() {
            if let Ok(l) = result {
//                println!("{:?}", l);
                //let mut tokenizer = Tokenizer::new(l.to_string());
                let tokens = Tokenizer::tokenize(l.to_string());
                println!("{:?}", tokens);
            }
//            println!("{:?}", l);
//            println!("end")
        }
    }
}
