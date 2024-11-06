use crate::ast::{BinaryOp, ComparisonOp, Expr, Statement, Type};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Program {
    basic_blocks: Vec<BasicBlock>,
    entry_block: usize,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub predecessors: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Binary {
        op: BinaryOp,
        left: String,
        right: String,
    },
    Comparison {
        op: ComparisonOp,
        left: String,
        right: String,
    },
    Phi {
        incoming: Vec<(BasicBlock, String)>,
    },
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub result: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Jump(String),
    Branch {
        condition: String,
        true_block: String,
        false_block: String,
    },
}

#[derive(Debug)]
struct TranslationContext {
    current_var: usize,
    current_block: usize,
    current_block_instructions: Vec<Instruction>,
    blocks: Vec<BasicBlock>,
    variable_versions: HashMap<String, String>,
    current_terminator: Option<Terminator>,
}

impl TranslationContext {
    fn new() -> Self {
        TranslationContext {
            current_var: 0,
            current_block: 0,
            current_block_instructions: Vec::new(),
            blocks: Vec::new(),
            variable_versions: HashMap::new(),
            current_terminator: None,
        }
    }

    fn new_var(&mut self) -> String {
        self.current_var += 1;
        format!("&{}", self.current_var)
    }

    fn new_block(&mut self) -> String {
        self.current_block += 1;
        format!("block{}", self.current_block)
    }

    fn finish_block(&mut self, terminator: Terminator) -> String {
        let block_name = format!("block{}", self.current_block);
        let instructions = std::mem::take(&mut self.current_block_instructions);

        self.blocks.push(BasicBlock {
            name: block_name.clone(),
            instructions,
            terminator,
            predecessors: Vec::new(),
        });

        self.new_block()
    }
}
