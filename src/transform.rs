use crate::node::*;
use crate::utils::*;


impl PseudoInst {
    fn transform_to(self) -> Vec<Instruction> {
        let Instruction(i, exprs) = self.0;
        let mut exprs = exprs.iter();
        if i == "la" {
            let rd = next_(&mut exprs);
            let sym = next_(&mut exprs);
            return vec![
                Instruction("auipc".to_string(), vec![rd.clone(), sym.clone()]),
                Instruction("addi".to_string(), vec![rd.clone(), rd, sym]),
            ];
        }
        if ["lb", "lh", "lw", "ld"].contains(&&*i) {
            let rd = next_(&mut exprs);
            let sym = next_(&mut exprs);
            return vec![
                Instruction("auipc".to_string(), vec![rd.clone(), sym.clone()]),
                Instruction(i, vec![rd.clone(), combinat_offset(sym, rd)]),
            ];
        }
        if ["sb", "sh", "sw", "sd",
            "flw", "fld",
            "fsw", "fsd"].contains(&&*i) {
            let rd = next_(&mut exprs);
            let sym = next_(&mut exprs);
            let rt = next_(&mut exprs);
            return vec![
                Instruction("auipc".to_string(), vec![rt.clone(), sym.clone()]),
                Instruction(i, vec![rd.clone(), combinat_offset(sym, rt)]),
            ];
        }
        if i == "nop" {
            return vec![
                Instruction("addi".to_string(), vec![
                    create_reg(0),
                    create_reg(0),
                    create_imm("0")]),
            ];
        }
        // li rd, immediate
        if i == "mv" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("addi".to_string(), vec![rd, rs, create_imm("0")])];
        }
        if i == "not" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("xori".to_string(), vec![rd, rs, create_imm("-1")])];
        }
        if i == "neg" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("sub".to_string(), vec![rd, create_reg(0), rs])];
        }
        if i == "negw" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("subw".to_string(), vec![rd, create_reg(0), rs])];
        }
        if i == "sext.w" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("addiw".to_string(), vec![rd, rs, create_imm("0")])];
        }
        if i == "seqz" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("sltiu".to_string(), vec![rd, rs, create_imm("1")])];
        }
        if i == "snez" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("sltu".to_string(), vec![rd, create_reg(0), rs])];
        }
        if i == "sltz" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("slt".to_string(), vec![rd, rs, create_reg(0)])];
        }
        if i == "sgtz" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction("slt".to_string(), vec![rd, create_reg(0), rs])];
        }
        // float instruction
        // ...
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