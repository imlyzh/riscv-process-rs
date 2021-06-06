use pest::iterators::{Pair, Pairs};
use pest::{error::Error, Parser};
use pest_derive::*;

use crate::node::*;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct RiscVAsm {}

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}

impl ParseFrom<Rule> for Register {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::registers);
        Register::from(pair.as_str())
    }
}

impl ParseFrom<Rule> for RfKeyword {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::rf_keyword);
        RfKeyword::from(pair.as_str())
    }
}

impl ParseFrom<Rule> for Rf {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::rf);
        let mut pairs = pair.into_inner();
        let keyword = pairs.next().unwrap();
        let symbol = pairs.next().unwrap();
        Self(RfKeyword::parse_from(keyword), Symbol::parse_from(symbol))
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::symbol);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::num => Self(pair.as_str().to_string(), 0),
            Rule::sym => {
                let mut pairs = pair.into_inner();
                let sym = pairs.next().unwrap();
                let offset = pairs.next();
                if offset.is_none() {
                    return Self(sym.as_str().to_string(), 0);
                }
                let offset = offset.unwrap().as_str().parse().unwrap();
                Self(sym.as_str().to_string(), offset)
            }
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for Offset {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::offset);
        let mut pairs = pair.into_inner();
        let pair = pairs.next().unwrap();
        let right = pairs.next().map(Register::parse_from);
        match pair.as_rule() {
            Rule::symbol => Offset::Imm(Symbol::parse_from(pair), right),
            Rule::rf => Offset::Rf(Rf::parse_from(pair), right),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for InstExpr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::inst_expr);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::registers => InstExpr::Reg(Register::parse_from(pair)),
            Rule::offset => InstExpr::RealTimeOffset(Offset::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for Instruction {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::inst);
        let mut pairs = pair.into_inner();
        let inst = pairs.next().unwrap().as_str();
        let exprs = pairs.map(InstExpr::parse_from);
        Self(inst.to_string(), exprs.collect())
    }
}

impl ParseFrom<Rule> for PseudoInst {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::pseudo_inst);
        let pair = pair.into_inner().next().unwrap();
        if let Rule::generic_pseudo_inst = pair.as_rule() {
            let mut pairs = pair.into_inner();
            let inst = pairs.next().unwrap().as_str();
            let exprs = pairs.map(InstExpr::parse_from);
            PseudoInst(Instruction(inst.to_string(), exprs.collect()))
        } else if let Rule::io_pinst = pair.as_rule() {
            let mut pairs = pair.into_inner();
            let inst = pairs.next().unwrap().as_str();
            let reg = pairs.next().unwrap();
            let sym = pairs.next().unwrap();
            let reg = InstExpr::Reg(Register::parse_from(reg));
            let sym = InstExpr::RealTimeOffset(Offset::Imm(Symbol::parse_from(sym), None));
            PseudoInst(Instruction(inst.to_string(), vec![reg, sym]))
        } else {
            unreachable!()
        }
    }
}

impl ParseFrom<Rule> for Expr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::expr);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::sym => Expr::Sym(pair.as_str().to_string()),
            Rule::str => Expr::Str(pair.as_str().to_string()),
            Rule::num => Expr::Num(pair.as_str().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for Pseudo {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::pseudo);
        let pair = pair.into_inner().next().unwrap();
        let mut pairs = pair.into_inner();
        let pseudo_op = pairs.next().unwrap().as_str();
        let exprs = pairs.map(Expr::parse_from);
        Self(pseudo_op.to_string(), exprs.collect())
    }
}

impl ParseFrom<Rule> for Label {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::label);
        let pair = pair.into_inner().next().unwrap();
        Self(Symbol::parse_from(pair))
    }
}

impl ParseFrom<Rule> for Node {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::line);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::inst => Node::Inst(Instruction::parse_from(pair)),
            Rule::pseudo_inst => Node::PseudoInst(PseudoInst::parse_from(pair)),
            Rule::pseudo => Node::PseudoOps(Pseudo::parse_from(pair)),
            Rule::label => Node::Label(Label::parse_from(pair)),
            _ => unreachable!(),
        }
    }
}


pub fn parse(i: &str) -> Result<Vec<Node>, Error<Rule>> {
    let r: Result<Vec<Pairs<Rule>>, Error<Rule>> = i
        .split('\n')
        .map(str::trim)
        .map(|x| RiscVAsm::parse(Rule::line, x))
        .collect();
    let r = r?;
    let r = r
        .into_iter()
        .flatten()
        .filter(|pair| pair.clone().into_inner().next().is_some())
        .map(Node::parse_from)
        .collect();
    Ok(r)
}
