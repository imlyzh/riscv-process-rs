use std::convert::TryFrom;


#[derive(Debug)]
pub enum Node {
    Label(Label),
    Inst(Instruction),
    PseudoOps(Pseudo)
}

#[derive(Debug)]
pub enum RawNode {
    Label(Label),
    Inst(Instruction),
    PseudoInst(PseudoInst),
    PseudoOps(Pseudo)
}

#[derive(Debug)]
pub struct Pseudo(pub String, pub Vec<Expr>);

#[derive(Debug)]
pub enum Expr {
    Str(String),
    Num(i64),
    Sym(Symbol)
}

#[derive(Debug)]
pub struct PseudoInst(pub Instruction);

#[derive(Debug)]
pub struct Instruction(pub String, pub Vec<InstExpr>);

#[derive(Debug)]
pub enum InstExpr {
    Reg(Register),
    RealTimeOffset(Offset),
}

#[derive(Debug)]
pub enum Offset {
    Rf(Rf, Option<Register>),
    Address(Symbol, Option<Register>),
}

#[derive(Debug)]
pub struct Rf(pub RfKeyword, pub Symbol);

#[repr(C)]
#[derive(Debug)]
pub enum RfKeyword {
    Hi,
    Lo,
    PcrelHi,
    PcrelLo,
    TprelHiE,
    TprelLoE,
    GotPcrelHi,
    TlsIePcrelHiS,
    TlsGdPcrelHis,
}

macro_rules! transform_rf_keyword {
    ($i: expr, $s: expr, $r: expr) => {
        if $i == $s {
            return Some($r)
        }
    };
}

impl From<&str> for RfKeyword {
    fn from(i: &str) -> Self {
        RfKeyword::from_sym(i).expect("invalid riscv assembler relocation functions")
    }
}

impl RfKeyword {

    #[inline]
    fn from_sym(i: &str) -> Option<Self> {
        transform_rf_keyword!(i, "hi", RfKeyword::Hi);
        transform_rf_keyword!(i, "lo", RfKeyword::Lo);
        transform_rf_keyword!(i, "pcrel_hi", RfKeyword::PcrelHi);
        transform_rf_keyword!(i, "pcrel_lo", RfKeyword::PcrelLo);
        transform_rf_keyword!(i, "tprel_hi_e", RfKeyword::TprelHiE);
        transform_rf_keyword!(i, "tprel_lo_e", RfKeyword::TprelLoE);
        transform_rf_keyword!(i, "got_pcrel_hi", RfKeyword::GotPcrelHi);
        transform_rf_keyword!(i, "tls_ie_pcrel_hi_s", RfKeyword::TlsIePcrelHiS);
        transform_rf_keyword!(i, "tls_gd_pcrel_hi_s", RfKeyword::TlsGdPcrelHis);
        None
    }
}

#[derive(Debug)]
pub struct Symbol (pub String, pub u64);

#[derive(Debug)]
pub struct Register(u8);

impl Register {
    #[inline]
    pub fn new(i: u8) -> Self {
        assert!(i < 32);
        Register(i)
    }

    #[inline]
    pub fn get_reg_num(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn from_sym(i: &str) -> Option<Self> {
        let record = [
            vec!["x0", "zero"],
            vec!["x1", "ra"],
            vec!["x2", "sp"],
            vec!["x3", "gp"],
            vec!["x4", "tp"],
            vec!["x5", "t0"],
            vec!["x6", "t1"],
            vec!["x7", "t2"],
            vec!["x8", "s0", "fp"],
            vec!["x9", "s1"],
            vec!["x10", "a0"],
            vec!["x11", "a1"],
            vec!["x12", "a2"],
            vec!["x13", "a3"],
            vec!["x14", "a4"],
            vec!["x15", "a5"],
            vec!["x16", "a6"],
            vec!["x17", "a7"],
            vec!["x18", "s2"],
            vec!["x19", "s3"],
            vec!["x20", "s4"],
            vec!["x21", "s5"],
            vec!["x22", "s6"],
            vec!["x23", "s7"],
            vec!["x24", "s8"],
            vec!["x25", "s9"],
            vec!["x26", "s10"],
            vec!["x27", "s11"],
            vec!["x28", "t3"],
            vec!["x29", "t4"],
            vec!["x30", "t5"],
            vec!["x31", "t6"],
            ];
        for (line, item) in record.iter().enumerate() {
            for reg in item.iter() {
                if i == *reg {
                    return Some(Register::new(line as u8));
                } 
            }
        }
        None
    }
}


impl From<&str> for Register {
    fn from(i: &str) -> Self {
        Register::from_sym(i).expect("invalid riscv register symbol")
    }
}

#[derive(Debug)]
pub struct Label (pub Symbol);
