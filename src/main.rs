use std::env;
use std::fs::File;
use std::io::prelude::*;
mod assembler;
use assembler::Assembler;
mod insts;
mod tokenizer;
mod common;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数エラー");
        return;
    }
    println!("{:?}", args);
    let filename = &args[1];
    //let mut f = File::open(filename).expect("xxxx");
    //let mut contents = String::new();
    //f.read_to_string(&mut contents).expect("xx");
    //println!("{}", contents);
    let mut assembler = Assembler::new();
    assembler.assemble(
        //        "/home/ttsurumi/workspace/famiasm/src/main.rs".to_string(),
        "sample/giko005.asm".to_string(),
        "out.bin".to_string(),
    );
}
