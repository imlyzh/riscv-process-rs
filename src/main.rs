mod node;
mod parser;

use parser::parse;

fn main() {
    let s = include_str!("../test/test1.asm");
    let r = parse(s);
    println!("{:?}", r);
}
