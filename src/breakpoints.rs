use nix::unistd::Pid;

pub struct Breakpoint{
    pid:Pid,
    address:*mut i64,
    is_enabled:bool,
    original_byte:u8,
}

impl Breakpoint{
    pub fn new(pid:Pid,address:*mut i64)->Self{
        Breakpoint{
            pid:pid,
            address:address,
            is_enabled:false,
            original_byte:0x0
        }
    }

    pub fn get_address(&self)->*mut i64{
        self.address
    }

    pub fn is_enabled(&self)->bool{
        self.is_enabled
    }

    pub fn enable(){

    }

    pub fn disable(){
        
    }
}