use serde::{Deserialize, Serialize};
use crate::{node::*, utils::{get_num, get_str, get_sym}};


#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct LabelInfo {
    label_name: String,
    start: usize,
    end: usize
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct PosInfo {
    pub file: usize,
    pub line: usize,
    pub col: usize,
    pub start: usize,
    pub end: usize
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BlockMap{
    pub file_list: Vec<String>,
    pub type_info_table: Vec<TypeBind>,
    pub label_table: Vec<LabelInfo>,
    pub pos_table: Vec<PosInfo>,
    pub inst_table: Vec<Node>
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct TypeBind {
    pub label: String,
    pub type_name: String,
}

pub fn block(i: Vec<Node>) -> BlockMap {
    let mut file_list: Vec<String> = vec![];
    let mut type_info_table: Vec<TypeBind> = vec![];
    let mut label_table: Vec<LabelInfo> = vec![];
    let mut pos_table: Vec<PosInfo> = vec![];
    let mut inst_table: Vec<Node> = vec![];
    let mut offset: usize = 0;
    let mut label_record: Option<(String, usize)> = None;
    let mut pos_record: Option<(usize, usize, usize, usize)> = None;
    for i in i {
        if let Node::Label(Label(Symbol(i, _))) = i.clone() {
            if let Some((label, start)) = label_record.clone() {
                label_table.push(LabelInfo {
                    label_name: label,
                    start,
                    end: offset,
                });
            }
            if let Some((file, line, col, start)) = pos_record.clone() {
                pos_table.push(PosInfo {
                file, line, col, start,
                    end: offset,
                });
            }
            label_record = Some((i, offset));
            continue;
        }
        if let Node::PseudoOps(Pseudo(op, exprs)) = i.clone() {
            if op.as_str() == "loc" {
                if let Some((file, line, col, start)) = pos_record.clone() {
                    let end = offset;
                    pos_table.push(PosInfo { file, line, col, start, end });
                }
                let file = get_num(exprs.get(0).unwrap()).parse::<usize>().unwrap();
                let line = get_num(exprs.get(1).unwrap()).parse::<usize>().unwrap();
                let col = get_num(exprs.get(2).unwrap()).parse::<usize>().unwrap();
                pos_record = Some((file, line, col, offset));
                continue;
            }
            if op.as_str() == "file" {
                match exprs.len() {
                    1 => {
                        let file_name = get_str(exprs.get(0).unwrap());
                        file_list.insert(0, file_name)
                    }
                    2 => {
                        let offset = get_num(exprs.get(0).unwrap()).parse::<usize>().unwrap();
                        let file_name = get_str(exprs.get(1).unwrap());
                        file_list.insert(offset, file_name)
                    }
                    _ => unreachable!("wtf")
                }
                continue;
            }
            if op.as_str() == "type" {
                let label = get_sym(exprs.get(0).unwrap());
                let type_name = get_sym(exprs.get(1).unwrap());
                type_info_table.push(TypeBind { label, type_name });
                continue;
            }
        }

        inst_table.push(i);
        offset+=1;
    }
    BlockMap{
        file_list,
        type_info_table,
        label_table,
        pos_table,
        inst_table,
    }
}
