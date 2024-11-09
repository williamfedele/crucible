use crate::ast::{BinaryOp, Expr, Statement};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    // Expr::Integer
    Constant {
        result: String,
        value: i64,
    },
    // Expr::Binary
    Binary {
        result: String,
        op: BinaryOp,
        left: String,
        right: String,
    },
}
#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub variables: HashMap<String, i64>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: Vec::new(),
            variables: HashMap::new(), // track number of variable versions
        }
    }
}

fn translate_literal(value: i64, ir: &mut Program, target: Option<&str>) -> String {
    let result = if let Some(name) = target {
        gen_name(name, ir)
    } else {
        format!("{}", value)
    };

    if target.is_some() {
        ir.instructions.push(Instruction::Constant {
            result: result.clone(),
            value,
        });
    }
    result
}

fn translate_expr(expr: &Expr, ir: &mut Program, target: Option<&str>) -> String {
    match expr {
        Expr::Integer(value) => translate_literal(*value, ir, target),
        Expr::Variable(name) => format!("{}.{}", name, ir.variables.get(name).unwrap()),
        Expr::Binary { op, left, right } => {
            let left_var = match left.as_ref() {
                Expr::Integer(value) => translate_literal(*value, ir, None),
                _ => translate_expr(&left, ir, None),
            };
            let right_var = match right.as_ref() {
                Expr::Integer(value) => translate_literal(*value, ir, None),
                _ => translate_expr(&right, ir, None),
            };
            let result = if let Some(name) = target {
                gen_name(name, ir)
            } else {
                gen_name("bin", ir)
            };

            ir.instructions.push(Instruction::Binary {
                result: result.clone(),
                op: op.clone(),
                left: left_var,
                right: right_var,
            });
            result
        }
    }
}

fn gen_name(name: &str, ir: &mut Program) -> String {
    let counter = ir.variables.entry(name.to_string()).or_insert(0);
    *counter += 1;
    format!("{}.{}", name, counter)
}

pub fn lower(statements: Vec<Statement>) -> Program {
    let mut ir = Program::new();

    for stmt in statements {
        match stmt {
            Statement::Let { name, value, .. } => {
                translate_expr(&value, &mut ir, Some(&name));
            }
            Statement::Assignment { target, value } => {
                translate_expr(&value, &mut ir, Some(&target));
            }
        }
    }

    ir
}

fn constant_folding(program: &mut Program) {
    let mut known_constants: HashMap<String, i64> = HashMap::new();
    let mut modified = true;

    while modified {
        modified = false;
        let mut i = 0;

        while i < program.instructions.len() {
            let instruction = program.instructions[i].clone();
            match instruction {
                Instruction::Constant { result, value } => {
                    known_constants.insert(result.clone(), value);
                }
                Instruction::Binary {
                    result,
                    op,
                    left,
                    right,
                } => {
                    let left_val = known_constants
                        .get(&left)
                        .copied()
                        .or_else(|| left.parse::<i64>().ok());
                    let right_val = known_constants
                        .get(&right)
                        .copied()
                        .or_else(|| right.parse::<i64>().ok());

                    if let (Some(left_val), Some(right_val)) = (left_val, right_val) {
                        let new_value = match op {
                            BinaryOp::Add => left_val + right_val,
                            BinaryOp::Subtract => left_val - right_val,
                            BinaryOp::Multiply => left_val * right_val,
                            BinaryOp::Divide => left_val / right_val,
                        };
                        program.instructions[i] = Instruction::Constant {
                            result: result.clone(),
                            value: new_value,
                        };
                        known_constants.insert(result.clone(), new_value);
                        modified = true;
                    }
                }
            }
            i += 1;
        }
    }
}

fn dead_code_elimination(program: &mut Program) {
    let mut uses: HashMap<String, usize> = HashMap::new();

    for inst in &program.instructions {
        match inst {
            Instruction::Binary {
                result,
                op,
                left,
                right,
            } => {
                *uses.entry(left.clone()).or_default() += 1;
                *uses.entry(right.clone()).or_default() += 1;
            }
            _ => {}
        }
    }

    program.instructions.retain(|inst| match inst {
        Instruction::Constant { result, .. } | Instruction::Binary { result, .. } => {
            uses.get(result).copied().unwrap_or(0) > 0
        }
    })
}

pub fn optimize(program: &mut Program) {
    println!("\nOriginal IR: {:?}", program.instructions);
    dead_code_elimination(program);
    println!("\nDead Code IR: {:?}", program.instructions);
    constant_folding(program);
    println!("\nConstant Fold IR: {:?}", program.instructions);
}
