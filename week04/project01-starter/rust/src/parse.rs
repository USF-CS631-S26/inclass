// parse.rs - parsing and parse tree construction
//
// # Parser EBNF
//
// program    ::= expression EOT
// expression ::= operand (operator operand)*
// operand    ::= intlit | '-' operand
// operator   ::= '+' | '-'

use std::process;

use crate::scan::{ScanTable, TokenType};

// ============================================================================
// Parse Tree
// ============================================================================

/// Operator types
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
}

impl Operator {
    pub fn name(&self) -> &str {
        match self {
            Operator::Plus => "PLUS",
            Operator::Minus => "MINUS",
            Operator::Mult => "MULT",
            Operator::Div => "DIV",
        }
    }
}

/// Parse tree node types
#[allow(dead_code)]
pub enum ParseNode {
    IntVal { value: u32 },
    Oper1 { oper: Operator, operand: Box<ParseNode> },
    Oper2 { oper: Operator, left: Box<ParseNode>, right: Box<ParseNode> },
}

impl ParseNode {
    /// Create a new IntVal node
    pub fn new_intval(value: u32) -> Self {
        ParseNode::IntVal { value }
    }

    /// Create a new Oper1 node
    pub fn new_oper1(oper: Operator, operand: ParseNode) -> Self {
        ParseNode::Oper1 {
            oper,
            operand: Box::new(operand),
        }
    }

    /// Create a new Oper2 node
    pub fn new_oper2(oper: Operator, left: ParseNode, right: ParseNode) -> Self {
        ParseNode::Oper2 {
            oper,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Print indentation for parse tree output
fn parse_tree_print_indent(level: usize) {
    for _ in 0..(level * 2) {
        print!(".");
    }
}

/// Print a parse tree node recursively
fn parse_tree_print_expr(node: &ParseNode, level: usize) {
    parse_tree_print_indent(level);
    print!("EXPR ");

    match node {
        ParseNode::IntVal { value } => {
            println!("INTVAL {}", value);
        }
        ParseNode::Oper1 { oper, operand } => {
            println!("OPER1 {}", oper.name());
            parse_tree_print_expr(operand, level + 1);
        }
        ParseNode::Oper2 { oper, left, right } => {
            println!("OPER2 {}", oper.name());
            parse_tree_print_expr(left, level + 1);
            parse_tree_print_expr(right, level + 1);
        }
    }
}

/// Print the entire parse tree
pub fn parse_tree_print(node: &ParseNode) {
    parse_tree_print_expr(node, 0);
}

// ============================================================================
// Parser
// ============================================================================

/// Report a parse error and exit
fn parse_error(msg: &str) -> ! {
    eprintln!("parse_error: {}", msg);
    process::exit(-1);
}

/// Parse a complete program: expression EOT
pub fn parse_program(scan_table: &mut ScanTable) -> ParseNode {
    let expr = parse_expression(scan_table);

    if !scan_table.accept(TokenType::Eot) {
        parse_error("Expecting EOT");
    }

    expr
}

/// Parse an expression: operand (operator operand)*
fn parse_expression(scan_table: &mut ScanTable) -> ParseNode {
    let mut left = parse_operand(scan_table);

    loop {
        let token = scan_table.get(0);
        match token.token_type() {
            TokenType::Plus => {
                scan_table.accept_any();
                let right = parse_operand(scan_table);
                left = ParseNode::new_oper2(Operator::Plus, left, right);
            }
            TokenType::Minus => {
                scan_table.accept_any();
                let right = parse_operand(scan_table);
                left = ParseNode::new_oper2(Operator::Minus, left, right);
            }
            _ => break,
        }
    }

    left
}

/// Parse an operand: intlit | '-' operand
fn parse_operand(scan_table: &mut ScanTable) -> ParseNode {
    if scan_table.accept(TokenType::IntLit) {
        let token = scan_table.get(-1);
        let value: u32 = token.value().parse().unwrap_or_else(|_| {
            parse_error("Invalid integer literal");
        });
        ParseNode::new_intval(value)
    } else if scan_table.accept(TokenType::Minus) {
        let operand = parse_operand(scan_table);
        ParseNode::new_oper1(Operator::Minus, operand)
    } else {
        parse_error("Bad operand");
    }
}
