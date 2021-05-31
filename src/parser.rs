use pest::{Parser, error::Error};
use pest::iterators::{Pair, Pairs};
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
        Register::from(pair.as_str())
    }
}

impl ParseFrom<Rule> for Rf {
    fn parse_from(pair: Pair<Rule>) -> Self {
        todo!()
    }
}

impl ParseFrom<Rule> for Symbol {
    fn parse_from(pair: Pair<Rule>) -> Self {
        todo!()
    }
}

impl ParseFrom<Rule> for Offset {
    fn parse_from(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        let pair = pairs.next().unwrap();
        let right = pairs.next().map(Register::parse_from);
        match pair.as_rule() {
            Rule::symbol => Offset::Address(Symbol::parse_from(pair), right),
            Rule::rf => Offset::Rf(Rf::parse_from(pair), right),
            _ => unreachable!()
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
            _ => unreachable!()
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
        todo!()
    }
}

impl ParseFrom<Rule> for Pseudo {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::pseudo);
        todo!()
    }
}

impl ParseFrom<Rule> for Label {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::label);
        let mut pair = pair.into_inner().next().unwrap();
        Self(Symbol::parse_from(pair))
    }
}

impl ParseFrom<Rule> for RawNode {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::line);
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::inst => RawNode::Inst(Instruction::parse_from(pair)),
            Rule::pseudo_inst => RawNode::PseudoInst(PseudoInst::parse_from(pair)),
            Rule::pseudo => RawNode::PseudoOps(Pseudo::parse_from(pair)),
            Rule::label => RawNode::Label(Label::parse_from(pair)),
            _ => unreachable!()
        }
    }
}


pub fn parse(i: &str) -> Result<Vec<RawNode>, Error<Rule>> {
    let r: Result<Vec<Pairs<Rule>>, Error<Rule>> = i.split('\n')
        .map(str::trim)
        .map(|x| RiscVAsm::parse(Rule::unit, x)).collect();
    let r = r?;
    let r = r.iter()
        .flat_map(|f| f.clone().map(RawNode::parse_from)).collect();
    Ok(r)
}