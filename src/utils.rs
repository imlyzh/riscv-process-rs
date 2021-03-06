use crate::node::*;


#[inline]
pub fn get_num(i: &Expr) -> String {
    match i {
        Expr::Num(i) => i.clone(),
        _ => unreachable!("What is {:?}?", i),
    }
}

#[inline]
pub fn get_str(i: &Expr) -> String {
    match i {
        Expr::Str(i) => i.clone(),
        _ => unreachable!("What is {:?}?", i),
    }
}

#[inline]
pub fn get_sym(i: &Expr) -> String {
    match i {
        Expr::Sym(i) => i.clone(),
        _ => unreachable!("What is {:?}?", i),
    }
}

/*
use std::slice::Iter;

#[inline]
pub fn next_(i: &mut Iter<InstExpr>) -> InstExpr {
    i.next().unwrap().clone()
}

#[inline]
pub fn combinat_offset(i: InstExpr, i1: InstExpr) -> InstExpr {
    let reg = if let InstExpr::Reg(x) = i1 {
        x
    } else {
        unreachable!()
    };
    if let InstExpr::RealTimeOffset(x) = i {
        let r = match x {
            Offset::Rf(v, _) => Offset::Rf(v, Some(reg)),
            Offset::Imm(v, _) => Offset::Imm(v, Some(reg)),
        };
        return InstExpr::RealTimeOffset(r);
    }
    unreachable!()
}

#[inline]
pub fn create_reg(i: u8) -> InstExpr {
    InstExpr::Reg(Register::new(i))
}

#[inline]
pub fn create_imm(i: &str) -> InstExpr {
    InstExpr::RealTimeOffset(Offset::Imm(Symbol(i.to_string(), 0), None))
}
// */
