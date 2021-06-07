use crate::{node::*, utils::get_num};

#[derive(Debug, Clone)]
pub struct LabelInfo {
    label_name: String,
    start: usize,
    end: usize
}

#[derive(Debug, Clone)]
pub struct PosInfo {
    file: usize,
    line: usize,
    col: usize,
    start: usize,
    end: usize
}

#[derive(Debug, Clone)]
pub struct BlockMap(pub Vec<LabelInfo>, pub Vec<PosInfo>, Vec<Node>);

pub fn block(i: Vec<Node>) -> BlockMap {
    let mut label_info_table: Vec<LabelInfo> = vec![];
    let mut line_info_table: Vec<PosInfo> = vec![];
    let mut inst_table: Vec<Node> = vec![];
    let mut offset: usize = 0;
    let mut label_record: Option<(String, usize)> = None;
    let mut pos_record: Option<(usize, usize, usize, usize)> = None;
    for i in i {
        match i {
            Node::Label(Label(Symbol(i, _))) => {
                if let Some((label, start)) = label_record.clone() {
                    label_info_table.push(LabelInfo {
                        label_name: label,
                        start,
                        end: offset,
                    });
                }
                if let Some((file, line, col, start)) = pos_record.clone() {
                    line_info_table.push(PosInfo {
                    file, line, col, start,
                        end: offset,
                    });
                }
                label_record = Some((i, offset));
            }
            Node::PseudoOps(Pseudo(op, exprs)) if op.as_str() == "loc" => {
                // assert_eq!(exprs.len(), 3);
                if let Some((file, line, col, start)) = pos_record.clone() {
                    line_info_table.push(PosInfo {
                        file, line, col, start,
                            end: offset,
                        });
                }
                let file = get_num(exprs.get(0).unwrap()) as usize;
                let line = get_num(exprs.get(1).unwrap()) as usize;
                let col = get_num(exprs.get(2).unwrap()) as usize;
                pos_record = Some((file, line, col, offset));
            }
            _ => {
                inst_table.push(i);
                offset+=1;
            }
        }
    }
    BlockMap(label_info_table, line_info_table, inst_table)
}
