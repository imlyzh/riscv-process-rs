mod node;
mod parser;
mod transform;
mod block;
mod utils;

use parser::parse;
use block::block;
use serde_json::to_string;

fn main() {
    let s = include_str!("../test/test.asm");
    let r = parse(s);
    let r = r.unwrap();
    let r = block(r);
    let r = serde_json::to_string(&r).unwrap();
    println!("{}", r);
}
