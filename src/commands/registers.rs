use crate::utils::throw_custom_error;
use crate::{registers, utils};
use nix::unistd::Pid;
use std::error::Error;

pub fn handle_register_commands(command: &String, pid: i32) -> Result<(), Box<dyn Error>> {
    let items: Vec<String> = command.split(" ").map(|x| x.to_string()).collect();

    if items.len() == 2 {
        if items[1] == "dump" {
            let registers = registers::get_registers(Pid::from_raw(pid))?;
            for reg_name in registers::USER_REG_NAMES_X64 {
                let res = registers::get_register_value(registers, reg_name);
                match res {
                    Some(val) => {
                        println!("{:<10} : 0x{:X}", reg_name, val);
                    }
                    None => {
                        return Err(utils::throw_custom_error(format!(
                            "Invalid register: {}",
                            reg_name
                        )));
                    }
                }
            }
        }
    } else if items.len() == 3 {
        // reg get reg_name
        if items[1] == "get" {
            let reg_name = items[2].clone().to_ascii_lowercase();
            let registers = registers::get_registers(Pid::from_raw(pid))?;
            let reg_val = registers::get_register_value(registers, reg_name.as_str());
            match reg_val {
                Some(value) => {
                    println!("0x{:X}", value);
                }
                None => {
                    return Err(throw_custom_error("Invalid register".to_string()));
                }
            }
        }
    }else if items.len() == 4{
        // reg set reg_name 0x10
        // reg set reg_name 111
        if items[1] == "set"{
            let reg_name = items[2].clone().to_ascii_lowercase();
            let mut val_str = items[3].clone();
            if val_str.starts_with("0x"){
                val_str = val_str.replace("0x","");
                let value = u64::from_str_radix(&val_str,16)?;
                registers::set_register_value(Pid::from_raw(pid), &reg_name, value)?;
            }
            else{
                let value:u64 = val_str.parse()?;
                registers::set_register_value(Pid::from_raw(pid),&reg_name,value)?;
            }
        }
    }
    Ok(())
}
