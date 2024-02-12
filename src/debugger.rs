use super::{commands, input};
use nix::libc::{execl, ptrace};
use nix::libc::{fork, PTRACE_TRACEME};
use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use std::ffi::CString;
use std::io::{Error, ErrorKind};

#[derive(Clone)]
pub struct Debugger {
    file_path: String,
    child_pid: i32,
}

impl Debugger {
    pub fn new(file_path: String, pid: i32) -> Self {
        Debugger {
            file_path: file_path,
            child_pid: pid,
        }
    }

    pub fn get_child_path(&self)->String{
        self.file_path.clone()
    }

    pub fn continue_execution(&self) {
        let child_pid = Pid::from_raw(self.child_pid);
        let retval = ptrace::cont(child_pid, None);
        match retval {
            Ok(_) => {
                waitpid(child_pid, None);
            }
            Err(_) => {
                println!(
                    "Error, debugger::continue_execution -> {}",
                    Error::last_os_error()
                );
            }
        }
    }
}

fn execute_debugee(file_path: String) -> Result<(), std::io::Error> {
    unsafe {
        let retval = ptrace(PTRACE_TRACEME);
        if retval != 0 {
            return Err(Error::last_os_error());
        }
        let res = CString::new(file_path.clone());
        match res {
            Ok(path_cstring) => {
                let retval = execl(path_cstring.as_ptr(), path_cstring.as_ptr(), 0);
                if retval == -1 {
                    return Err(Error::last_os_error());
                }
            }
            Err(_) => {
                return Err(Error::new(ErrorKind::Other, "Failed to create CString"));
            }
        }
        Ok(())
    }
}

pub fn run_debugger(file_path: String) {
    unsafe {
        let pid = fork();
        match pid {
            0 => {
                // In the child process
                let res = execute_debugee(file_path);
                match res {
                    Ok(_) => {}
                    Err(err) => {
                        println!("debugger::run -> {}", err);
                    }
                }
            }
            _ => {
                // In the parent process

                println!("Started debugging {} with PID={}", file_path, pid);
                // Wait for the child process to start
                waitpid(Pid::from_raw(pid), None);
                run_debug_loop(file_path,pid);
            }
        }
    }
}

fn run_debug_loop(file_path:String,child_pid:i32) {
    let debugger:Debugger = Debugger::new(file_path.clone(), child_pid);
    loop {
        let inp = input::input_prompt("rupse".to_string());
        match inp {
            Ok(input) => {
                let command_handler = commands::CommandHandler::new(debugger.clone());
                command_handler.handle_command(input);
            }
            Err(error) => {
                println!("Error, debugger::run_debug_loop -> {}", error);
            }
        }
    }
}
