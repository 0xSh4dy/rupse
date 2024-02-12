mod debugger;
mod input;
mod commands;
mod breakpoints;
mod elfparser;
mod fileops;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2{
        println!("Usage: blazy_debugger <file_path>")
    }
    else{
        let file_path = args[1].clone();
        debugger::run_debugger(file_path);
    }
}
