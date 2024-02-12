use super::debugger::Debugger;
use super::elfparser::ELFParser;

pub struct CommandHandler {
    debugger: Debugger,
}

impl CommandHandler {
    pub fn new(debugger: Debugger) -> CommandHandler {
        CommandHandler {
            debugger: debugger,
        }
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
        } else if command == "continue" {
            self.debugger.continue_execution();
        } else if command == "list-fns" {
            let parser = ELFParser::new(self.debugger.get_child_path());
            parser.list_functions();
        }
    }
}
