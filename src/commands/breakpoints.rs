use crate::breakpoints::Breakpoint;
use crate::utils;
use colored::Colorize;
use nix::unistd::Pid;
use std::error::Error;

static mut BREAKPOINT_LIST: Vec<Breakpoint> = Vec::new();

pub fn print_help() {
    println!(
        "bpt set address : Sets a breakpoint at address(a valid address in hexadecimal format)"
    );
    println!("bpt del address : Deletes an existing breakpoint at address");
    println!("bpt show        : Lists all the breakpoints")
}
pub fn handle_breakpoint_commands(command: &String, pid: i32) -> Result<(), Box<dyn Error>> {
    let chunks: Vec<String> = command.split(' ').map(|x| x.to_string()).collect();
    if chunks.len() == 2 {
        let cmd = chunks[1].clone();
        if cmd == "show" {
            unsafe {
                for (idx, item) in BREAKPOINT_LIST.iter().enumerate() {
                    println!("BREAKPOINT-{} : 0x{:X}", idx + 1, item.get_address());
                }
            }
            return Ok(());
        }
    } else if chunks.len() == 3 {
        let cmd1 = chunks[1].clone();
        let mut address = chunks[2].clone();
        if !address.starts_with("0x") {
            return Err(utils::throw_custom_error(
                "Address must be a valid hexadecimal value starting with 0x".to_string(),
            ));
        }
        address = address.replace("0x", "");
        let addr: i64 = i64::from_str_radix(address.as_str(), 16)?;

        if cmd1 == "set" {
            unsafe {
                let bpt = BREAKPOINT_LIST.iter().find(|x| x.get_address() == addr);
                match bpt {
                    Some(_) => {
                        return Err(utils::throw_custom_error(
                            "Breakpoint already exists".to_string(),
                        ));
                    }
                    None => {}
                }
            }

            let mut bpt = Breakpoint::new(Pid::from_raw(pid), addr);
            bpt.enable()?;
            unsafe {
                BREAKPOINT_LIST.push(bpt);
                println!("{}", String::from("Breakpoint added").yellow());
            }
        } else if cmd1 == "del" {
            unsafe {
                let res = BREAKPOINT_LIST.iter().position(|x| x.get_address() == addr);
                match res {
                    Some(idx) => {
                        let bpt = &mut BREAKPOINT_LIST[idx];
                        bpt.disable()?;
                        BREAKPOINT_LIST.remove(idx);
                        let res = format!("Deleted BREAKPOINT-{} (0x{:X})", idx + 1, addr);
                        println!("{}", res.yellow());
                    }
                    None => {
                        return Err(utils::throw_custom_error(
                            "Breakpoint not found".to_string(),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}
