use crate::node::*;


impl PseudoInst {
    fn transform_to(self) -> Vec<Instruction> {
        let Instruction(i, exprs) = self.0;
        let mut exprs = exprs.iter();
        if i == "la" {
            let ra = exprs.next().unwrap();

        }
        todo!();
        unreachable!();
    }
}


impl RawNode {
    fn transform_to(self) -> Vec<Node> {
        match self {
            RawNode::Label(v) => vec![Node::Label(v)],
            RawNode::Inst(v) => vec![Node::Inst(v)],
            RawNode::PseudoOps(v) => vec![Node::PseudoOps(v)],
            RawNode::PseudoInst(v) => v.transform_to().into_iter()
                .map(Node::Inst)
                .collect(),
        }
    }
}