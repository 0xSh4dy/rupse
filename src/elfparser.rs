use super::fileops;
use super::function::Function;
use goblin::elf::{Elf, Sym};
use std::error::Error;
pub struct ELFParser {
    file_path: String,
}

impl ELFParser {
    pub fn new(file_path: String) -> ELFParser {
        ELFParser {
            file_path: file_path,
        }
    }

    pub fn get_function_info(&self, fn_name: &String) -> Result<Function, Box<dyn Error>> {
        let file = fileops::open_file(self.file_path.clone())?;
        let buffer = fileops::read_file(file);
        let elf = Elf::parse(&buffer)?;
        let strtab = elf.strtab;
        let symbol = elf
            .syms
            .iter()
            .find(|sym| strtab.get_at(sym.st_name).unwrap() == fn_name)
            .ok_or("Function not found")?;
        Ok(Function::new(
            fn_name.to_string(),
            symbol.st_value,
            symbol.st_size,
        ))
    }

    pub fn get_code(&self, function: &Function) -> Result<Vec<u8>, Box<dyn Error>> {
        let fn_address = function.get_address();
        let fn_size: usize = function.get_code_size() as usize;
        let file = fileops::open_file(self.file_path.clone())?;
        let file_data = fileops::read_file(file);
        let elf = Elf::parse(&file_data)?;

        // Retrieve the section header containing the function
        let sh = elf
            .section_headers
            .iter()
            .find(|sh| sh.sh_addr <= fn_address && fn_address < sh.sh_addr + sh.sh_size)
            .ok_or("Not found")?;

        let offset_within_section = fn_address - sh.sh_addr;
        let offset_within_elf = (sh.sh_offset + offset_within_section) as usize;
        let mut fn_code: Vec<u8> = vec![0; fn_size];
        fn_code.copy_from_slice(&file_data[offset_within_elf..(offset_within_elf + fn_size)]);
        Ok(fn_code)
    }

    pub fn list_functions(&self) -> Result<Vec<Function>, Box<dyn Error>> {
        let file = fileops::open_file(self.file_path.clone())?;
        let file_data = fileops::read_file(file);
        let elf = Elf::parse(&file_data)?;
        let strtab = elf.strtab;
        let mut fn_names: Vec<Function> = Vec::new();

        let symbols: Vec<Sym> = elf.syms.iter().filter(|sym| sym.is_function()).collect();

        for sym in symbols {
            let fn_name = strtab.get_at(sym.st_name).unwrap();
            let fn_address = sym.st_value;
            let fn_size = sym.st_size;
            if fn_address != 0 {
                fn_names.push(Function::new(fn_name.to_string(), fn_address, fn_size));
            }
        }
        Ok(fn_names)
    }
}
