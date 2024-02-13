#[derive(Clone)]
pub struct Function {
    name: String,
    address: u64,
    code_size: u64,
}

impl Function {
    pub fn new(name: String, address: u64, code_size: u64) -> Function {
        Function {
            name: name,
            address: address,
            code_size: code_size,
        }
    }

    pub fn get_address(&self) -> u64 {
        self.address
    }

    pub fn get_code_size(&self) -> u64 {
        self.code_size
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
