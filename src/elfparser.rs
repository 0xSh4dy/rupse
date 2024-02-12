pub struct ELFParser{
    file_path:String,
}

impl ELFParser{
    pub fn new(file_path:String)->ELFParser{
        ELFParser{
            file_path:file_path
        }
    }

    pub fn list_functions(&self){
    }
}