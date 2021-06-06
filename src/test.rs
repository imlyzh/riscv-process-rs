use crate::parser::parse;
use crate::transform::transform_to;

#[test]
fn test() {
    let s = include_str!("../test/test1.asm");
    let r = parse(s);
    let r = r.unwrap();
    let _r = transform_to(r);
}
