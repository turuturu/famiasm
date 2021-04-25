use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::insts::{Opcode, Addressing, Instruction};
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
        let inst = Instruction::new(Opcode::STY, Addressing::IndirectY, Some(0x11));
        let infile = fs::File::open(asmfilepath.to_string()).unwrap();
        let str : String = "ss".to_string();
        let inst : Instruction = str.into();
        //let mut out_buf = BufWriter::new(fs::File::create(binfilepath).unwrap());
        //let reader = BufReader::new(infile);
        //for line in reader.lines() {}
        for result in BufReader::new(infile).lines() {
            let l = result;
            println!("{:?}", l);
            println!("end")
        }
    }
}
