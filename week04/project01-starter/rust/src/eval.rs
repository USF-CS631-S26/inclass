// eval.rs - tree evaluation and value printing

use std::process;

use crate::parse::{ParseNode, Operator};

fn eval_error(msg: &str) -> ! {
    eprintln!("eval_error: {}", msg);
    process::exit(-1);
}

pub fn eval(node: &ParseNode) -> u32 {
    match node {
        ParseNode::IntVal { value } => *value,
        ParseNode::Oper1 { oper, operand } => {
            let v1 = eval(operand);
            match oper {
                Operator::Minus => v1.wrapping_neg(),
                _ => eval_error("Bad operator"),
            }
        }
        ParseNode::Oper2 { oper, left, right } => {
            let v1 = eval(left);
            let v2 = eval(right);
            match oper {
                Operator::Plus => v1.wrapping_add(v2),
                Operator::Minus => v1.wrapping_sub(v2),
                _ => eval_error("Bad operator"),
            }
        }
    }
}

pub fn eval_print(value: u32) {
    println!("{}", value as i32);
}
