mod node;
mod parser;
mod transform;
mod block;
mod utils;

use parser::parse;
use block::block;

fn main() {
    let s = include_str!("../test/test1.asm");
    let r = parse(s);
    let r = r.unwrap();
    let r = block(r);
    println!("{:?}", r);
}
