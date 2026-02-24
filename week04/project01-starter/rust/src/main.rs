// project01.rs - initial parsing implementation

mod scan;
mod parse;
mod eval;

use std::env;
use std::process;

use scan::ScanTable;
use parse::{parse_program, parse_tree_print};
use eval::{eval, eval_print};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: project01 <expression>");
        println!("  Example: project01 \"1 + 2\"");
        process::exit(-1);
    }

    let input = &args[1];

    // Scan input into token table
    let mut scan_table = ScanTable::new();
    scan_table.scan(input);
    scan_table.print();
    println!();

    // Parse
    let parse_tree = parse_program(&mut scan_table);
    parse_tree_print(&parse_tree);
    println!();

    // Eval
    let value = eval(&parse_tree);
    eval_print(value);
}
