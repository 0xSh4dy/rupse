use std::ffi::CStr;

use crate::registers;
use colored::Colorize;
use nix::libc::{siginfo_t, strsignal, SIGSEGV, SIGTRAP};

pub struct SignalHandler {
    siginfo: siginfo_t,
    child_pid: i32,
}

#[allow(non_camel_case_types)]
enum SigtrapCodes {
    TRAP_BRKPT = 0x1,
    TRAP_TRACE = 0x2,
    SI_KERNEL = 0x80,
}
impl SignalHandler {
    pub fn new(siginfo: siginfo_t, child_pid: i32) -> Self {
        SignalHandler {
            siginfo: siginfo,
            child_pid: child_pid,
        }
    }

    pub fn handle_signal(&self) -> Result<(), Box<dyn std::error::Error>> {
        let signo = self.siginfo.si_signo;
        match signo {
            SIGSEGV => {
                self.handle_sigsegv();
            }
            SIGTRAP => {
                self.handle_sigtrap()?;
            }
            _ => {
                self.handle_other_signals();
            }
        }

        Ok(())
    }

    fn handle_sigsegv(&self) {
        unsafe {
            let res = format!(
                "Program received SIGSEGV, segmentation fault: 0x{:X}",
                self.siginfo.si_addr() as u64
            );
            println!("{}", res.red());
        }
    }

    fn handle_sigtrap(&self) -> Result<(), Box<dyn std::error::Error>> {
        let code = self.siginfo.si_code;
        if code == (SigtrapCodes::SI_KERNEL as i32) || code == (SigtrapCodes::TRAP_BRKPT as i32) {
            // Set the breakpoint at the correct address
            let pc = registers::get_program_counter(self.child_pid)?;
            registers::set_program_counter(self.child_pid, pc - 1)?;
            let res = format!("Hit breakpoint at the address 0x{:X}", pc - 1);
            println!("{}", res.yellow());
        } 
        // else if code == (SigtrapCodes::TRAP_TRACE as i32) {
        // }
        Ok(())
    }

    fn handle_other_signals(&self) {
        unsafe {
            let cstr = CStr::from_ptr(strsignal(self.siginfo.si_signo));
            let mut res = cstr.to_string_lossy().to_string();
            res = format!("Received signal: {}", res);
            println!("{}", res.red());
        }
    }
}
