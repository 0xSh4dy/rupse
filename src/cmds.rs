use super::debugger::Debugger;
use super::disassembler;
use super::elfparser::ELFParser;
use colored::Colorize;
use super::commands::breakpoints;

pub struct CommandHandler {
    debugger: Debugger,
}

impl CommandHandler {
    pub fn new(debugger: Debugger) -> CommandHandler {
        CommandHandler { debugger: debugger }
    }
    pub fn handle_command(&self, mut command: String) {
        command.pop();
        if command == "exit" || command == "quit" {
            std::process::exit(0);
        } else if command == "help" {
            println!("exit/quit : Terminate the processes");
            println!("help : Display the help menu");
            println!("continue : Continue execution of the debugee");
            println!("list-fns : List all the functions");
            println!("disas function_name: Disassemble a function");

            // Breakpoint related commands
            breakpoints::print_help();
        } else if command == "continue" {
            self.debugger.continue_execution();
        } else if command == "list-fns" {
            let parser = ELFParser::new(self.debugger.get_child_path());
            let res = parser.list_functions();
            match res {
                Ok(functions) => {
                    for function in functions {
                        let fn_name = function.get_name();
                        let fn_address = format!("0x{:X}", function.get_address());
                        println!("{} : {}", fn_address.cyan(), fn_name.green());
                    }
                }
                Err(err) => {
                    println!("{}", format!("{}", err).red());
                }
            }
        } else if command.contains("disas") {
            let split: Vec<String> = command.split(' ').map(|x| x.to_string()).collect();
            if split.len() == 2 {
                if split[0] == "disas" {
                    let fn_name = split[1].clone();
                    let parser = ELFParser::new(self.debugger.get_child_path());
                    let res = parser.get_function_info(&fn_name);
                    match res {
                        Ok(function) => {
                            let res = parser.get_code(&function);
                            match res {
                                Ok(code) => disassembler::disassemble(
                                    code.as_slice(),
                                    function.get_address(),
                                    &fn_name,
                                ),
                                Err(err) => {
                                    println!("{}", format!("{}", err).red());
                                }
                            }
                        }
                        Err(err) => {
                            println!("{}", format!("{}", err).red());
                        }
                    }
                }
            }
        }
        else if command.contains("bpt"){
           let res = breakpoints::handle_breakpoint_commands(&command,self.debugger.get_child_pid());
           match res{
                Ok(())=>{},
                Err(err)=>{
                    let res = format!("Error: {}",err);
                    println!("{}",res.red());
                }
           }
        }
        else if command == "got"{

        }
    }
}
