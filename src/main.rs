mod node;
mod parser;
mod transform;
mod block;
mod utils;
mod analysis;

use std::{convert::identity, env, fs, io::{Read, Write}};

use block::block;
use parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        panic!("rpr <input.asm> <output.json> [--blocking]")
    }
    let mut input = fs::File::open(args.get(1).unwrap()).expect("input file is invalid");
    let mut output = fs::File::open(args.get(2).unwrap()).expect("output file is invalid");
    let is_blocking = args.get(2)
        .map(|x| x=="--blocking")
        .map_or(false, identity);
    
    let mut s = String::new();
    input.read_to_string(&mut s).expect("read input error");

    let r = parse(&s);
    let r = r.unwrap();
    let out_str;
    if is_blocking {
        let r = block(r);
        out_str = serde_json::to_string(&r).unwrap();
    } else {
        out_str = serde_json::to_string(&r).unwrap();
    }
    write!(output, "{}", out_str).expect("output error");
}
