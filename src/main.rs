mod node;
mod parser;
mod test;
mod transform;
mod utils;
mod block;

use parser::parse;
// use transform::transform_to;
use block::block;

fn main() {
    let s = include_str!("../test/test1.asm");
    let r = parse(s);
    println!("{:?}", r);
    let r = r.unwrap();
    let r = block(r);
    println!("{:?}", r);
}
