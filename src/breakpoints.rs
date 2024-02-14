use nix::unistd::Pid;
use nix::sys::ptrace;
use nix::libc::c_void;
use std::error::Error;

pub struct Breakpoint {
    pid: Pid,
    address: i64,
    original_byte: u8,
}

impl Breakpoint {
    pub fn new(pid: Pid, address: i64) -> Self {
        Breakpoint {
            pid: pid,
            address: address,
            original_byte: 0x0,
        }
    }

    pub fn get_address(&self) -> i64 {
        self.address
    }

    pub fn enable(&mut self) ->Result<(),Box<dyn Error>>{
        let retval = ptrace::read(self.pid,self.address as *mut c_void)?;
        self.original_byte = (retval & !0xff) as u8;
        // Clear the LSB and set int3 into it
        let val_with_int3:i64 = (retval& !0xff)|0xcc;
        unsafe{
            ptrace::write(self.pid,self.address as *mut c_void,val_with_int3 as *mut c_void);
        }
        Ok(())
    }

    pub fn disable(&mut self)->Result<(),Box<dyn Error>> {
        let retval = ptrace::read(self.pid,self.address as *mut c_void)?;
        let restored_val = ((retval & !0xff) | (self.original_byte as i64)) as i64;
        unsafe{
            ptrace::write(self.pid,self.address as *mut c_void,restored_val as *mut c_void);
        }
        Ok(())
    }
}
