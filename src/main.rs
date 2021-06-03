mod node;
mod parser;
mod transform;
mod utils;

use parser::parse;

fn main() {
    let s = include_str!("../test/test1.asm");
    let r = parse(s);
    println!("{:?}", r);
}
