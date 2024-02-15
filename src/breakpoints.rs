use nix::unistd::Pid;
use nix::sys::ptrace;
use nix::libc::c_void;
use std::error::Error;

pub struct Breakpoint {
    pid: Pid,
    address: u64,
    original_byte: u8,
    is_enabled:bool,
}

impl Breakpoint {
    pub fn new(pid: Pid, address: u64) -> Self {
        Breakpoint {
            pid: pid,
            address: address,
            original_byte: 0x0,
            is_enabled:true
        }
    }

    pub fn get_address(&self) -> u64 {
        self.address
    }

    pub fn is_bpt_enabled(&self)->bool{
        self.is_enabled
    }

    pub fn enable(&mut self) ->Result<(),Box<dyn Error>>{
        let retval = ptrace::read(self.pid,self.address as *mut c_void)?;
        self.original_byte = (retval & 0xff) as u8;
        // Clear the LSB and set int3 into it
        let val_with_int3:i64 = (retval& !0xff)|0xcc;
        unsafe{
            let _ = ptrace::write(self.pid,self.address as *mut c_void,val_with_int3 as *mut c_void);
        }
        self.is_enabled = true;
        Ok(())
    }

    pub fn disable(&mut self)->Result<(),Box<dyn Error>> {
        let retval = ptrace::read(self.pid,self.address as *mut c_void)?;
        let restored_val = ((retval & !0xff) | (self.original_byte as i64)) as i64;
        unsafe{
            let _ = ptrace::write(self.pid,self.address as *mut c_void,restored_val as *mut c_void);
        }
        self.is_enabled = false;
        Ok(())
    }
}
