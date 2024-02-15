mod debugger;
mod input;
mod cmd;
mod commands;
mod breakpoints;
mod elfparser;
mod fileops;
mod disassembler;
mod function;
mod dynamic;
mod utils;
mod registers;
mod signals;
mod backtrace;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2{
        println!("Usage: rupse <file_path>")
    }
    else{
        let file_path = args[1].clone();
        debugger::run_debugger(file_path);
    }
}
