pub mod node;
pub mod parser;
pub mod block;
mod utils;

use pyo3::prelude::*;

use crate::parser::parse;
use crate::block::block;

#[pymodule]
fn rprpy(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "get_nodes")]
    fn get_nodes(_py: Python, i: &str) -> PyResult<String> {    
        let r = parse(i);
        let r = r.expect("parser error");
        let out_str= serde_json::to_string(&r).unwrap();
        Ok(out_str)
    }

    #[pyfn(m, "get_blocks")]
    fn get_blocks(_py: Python, i: &str) -> PyResult<String> {
        let r = parse(i);
        let r = r.expect("parser error");
        let r = block(r);
        let out_str= serde_json::to_string(&r).unwrap();
        Ok(out_str)
    }
    Ok(())
}

mod test {
    #[test]
    fn test1() {

    }
}
