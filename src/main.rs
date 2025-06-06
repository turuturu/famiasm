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
        println!("引数エラー");
        return;
    }
    println!("{:?}", args);
    let _filename = &args[1];
    //let mut f = File::open(filename).expect("xxxx");
    //let mut contents = String::new();
    //f.read_to_string(&mut contents).expect("xx");
    //println!("{}", contents);
    let mut assembler = Assembler::new();
    assembler.assemble(
        //        "/home/ttsurumi/workspace/famiasm/src/main.rs".to_string(),
        "sample/giko005.asm".to_string(),
        "out.nes".to_string(),
    );
}
