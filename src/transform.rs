/*

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
        let load_global = ["lb", "lh", "lw", "ld"];
        if load_global.contains(&&*i) {
            let rd = next_(&mut exprs);
            let sym = next_(&mut exprs);
            return vec![
                Instruction("auipc".to_string(), vec![rd.clone(), sym.clone()]),
                Instruction(i, vec![rd.clone(), combinat_offset(sym, rd)]),
            ];
        }
        let store_global = ["sb", "sh", "sw", "sd", "flw", "fld", "fsw", "fsd"];
        if store_global.contains(&&*i) {
            let rd = next_(&mut exprs);
            let sym = next_(&mut exprs);
            let rt = next_(&mut exprs);
            return vec![
                Instruction("auipc".to_string(), vec![rt.clone(), sym.clone()]),
                Instruction(i, vec![rd.clone(), combinat_offset(sym, rt)]),
            ];
        }
        if i == "nop" {
            return vec![Instruction(
                "addi".to_string(),
                vec![create_reg(0), create_reg(0), create_imm("0")],
            )];
        }
        if i == "li" {
            let rd = next_(&mut exprs);
            let imm_str = next_(&mut exprs);
            let imm_str = if let InstExpr::RealTimeOffset(x) = imm_str {
                x
            } else {
                unreachable!()
            };
            let imm_str = if let Offset::Imm(x, _) = imm_str {
                x
            } else {
                unreachable!()
            };
            let imm_str = imm_str.0;
            let imm = imm_str.parse::<i32>().unwrap();
            let signed_i12: i32 = 2i32.pow(11); // [-2048, 2047]
            let i12 = signed_i12 * 2;
            if signed_i12 - 1 >= imm && imm >= -signed_i12 {
                return vec![Instruction(
                    "addi".to_string(),
                    vec![rd.clone(), create_imm(&imm_str)],
                )];
            } else {
                let mut lui_imm = imm / i12;
                let mut add_imm = imm % i12;
                if add_imm >= signed_i12 {
                    lui_imm += 1;
                    add_imm -= i12;
                }
                return vec![
                    Instruction(
                        "lui".to_string(),
                        vec![rd.clone(), create_imm(&lui_imm.to_string())],
                    ),
                    Instruction(
                        "addi".to_string(),
                        vec![rd.clone(), create_imm(&add_imm.to_string())],
                    ),
                ];
            }
        }
        if i == "mv" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction(
                "addi".to_string(),
                vec![rd, rs, create_imm("0")],
            )];
        }
        if i == "not" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction(
                "xori".to_string(),
                vec![rd, rs, create_imm("-1")],
            )];
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
        if i == "sext.b" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![
                Instruction(
                    "slli".to_string(),
                    vec![rd.clone(), rs, create_imm("XLEN - 8")],
                ),
                Instruction(
                    "srai".to_string(),
                    vec![rd.clone(), rd, create_imm("XLEN - 8")],
                ),
            ];
        }
        if i == "sext.h" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![
                Instruction(
                    "slli".to_string(),
                    vec![rd.clone(), rs, create_imm("XLEN - 16")],
                ),
                Instruction(
                    "srai".to_string(),
                    vec![rd.clone(), rd, create_imm("XLEN - 16")],
                ),
            ];
        }
        if i == "sext.w" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction(
                "addiw".to_string(),
                vec![rd, rs, create_imm("0")],
            )];
        }
        if i == "zext.b" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction(
                "andi".to_string(),
                vec![rd, rs, create_imm("255")],
            )];
        }
        if i == "zext.h" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![
                Instruction(
                    "slli".to_string(),
                    vec![rd.clone(), rs, create_imm("XLEN - 16")],
                ),
                Instruction(
                    "srli".to_string(),
                    vec![rd.clone(), rd, create_imm("XLEN - 16")],
                ),
            ];
        }
        if i == "zext.w" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![
                Instruction(
                    "slli".to_string(),
                    vec![rd.clone(), rs, create_imm("XLEN - 32")],
                ),
                Instruction(
                    "srli".to_string(),
                    vec![rd.clone(), rd, create_imm("XLEN - 32")],
                ),
            ];
        }
        if i == "seqz" {
            let rd = next_(&mut exprs);
            let rs = next_(&mut exprs);
            return vec![Instruction(
                "sltiu".to_string(),
                vec![rd, rs, create_imm("1")],
            )];
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
        // not implement float instructions now
        if i.starts_with("b") {
            if i.ends_with('z') {
                let rs = next_(&mut exprs);
                let offset = next_(&mut exprs);
                return match i.as_str() {
                    "beqz" => vec![Instruction(
                        "beq".to_string(),
                        vec![rs, create_reg(0), offset],
                    )],
                    "bnez" => vec![Instruction(
                        "bne".to_string(),
                        vec![rs, create_reg(0), offset],
                    )],
                    "blez" => vec![Instruction(
                        "bge".to_string(),
                        vec![create_reg(0), rs, offset],
                    )],
                    "bgez" => vec![Instruction(
                        "bge".to_string(),
                        vec![rs, create_reg(0), offset],
                    )],
                    "bltz" => vec![Instruction(
                        "blt".to_string(),
                        vec![rs, create_reg(0), offset],
                    )],
                    "bgtz" => vec![Instruction(
                        "blt".to_string(),
                        vec![create_reg(0), rs, offset],
                    )],
                    _ => unreachable!(),
                };
            } else {
                let rs = next_(&mut exprs);
                let rt = next_(&mut exprs);
                let offset = next_(&mut exprs);
                return match i.as_str() {
                    "bgt" => vec![Instruction("blt".to_string(), vec![rt, rs, offset])],
                    "ble" => vec![Instruction("bge".to_string(), vec![rt, rs, offset])],
                    "bgtu" => vec![Instruction("bltu".to_string(), vec![rt, rs, offset])],
                    "bleu" => vec![Instruction("bgeu".to_string(), vec![rt, rs, offset])],
                    _ => unreachable!(),
                };
            }
        }
        if i == "j" {
            let offset = next_(&mut exprs);
            return vec![Instruction("jal".to_string(), vec![create_reg(0), offset])];
        }
        if i == "jal" {
            let offset = next_(&mut exprs);
            return vec![Instruction("jal".to_string(), vec![create_reg(1), offset])];
        }
        if i == "jr" {
            let rs = next_(&mut exprs);
            return vec![Instruction(
                "jal".to_string(),
                vec![create_reg(0), rs, create_imm("0")],
            )];
        }
        if i == "jalr" {
            let rs = next_(&mut exprs);
            return vec![Instruction(
                "jalr".to_string(),
                vec![create_reg(1), rs, create_imm("0")],
            )];
        }
        if i == "ret" {
            return vec![Instruction(
                "jalr".to_string(),
                vec![create_reg(0), create_reg(1), create_imm("0")],
            )];
        }
        if i == "call" {
            let offset = next_(&mut exprs);
            return vec![Instruction("call".to_string(), vec![offset])];
        }
        if i == "tail" {
            let offset = next_(&mut exprs);
            return vec![Instruction("tail".to_string(), vec![offset])];
        }
        // not implement fence, and csr instructions now
        unreachable!();
    }
}

impl Node {
    fn transform_to(self) -> Vec<Node> {
        match self {
            Node::Label(v) => vec![Node::Label(v)],
            Node::Inst(v) => vec![Node::Inst(v)],
            Node::PseudoOps(v) => vec![Node::PseudoOps(v)],
            Node::PseudoInst(v) => v.transform_to().into_iter().map(Node::Inst).collect(),
        }
    }
}

//  */
