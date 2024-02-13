use colored::Colorize;
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};

pub fn disassemble(code: &[u8], address: u64, fn_name: &String) {
    let mut decoder = Decoder::with_ip(64, code, address, DecoderOptions::NONE);
    let mut instruction = Instruction::default();
    let mut formatter = NasmFormatter::new();
    formatter.options_mut().set_first_operand_char_index(10);

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        let instr_str = instruction.to_string();
        let instr_addr = format!("0x{:X}", instruction.ip());
        let offset = format!("<{}+{}>", fn_name, instruction.ip() - address);
        println!(
            "{:<20} {:<20} {}",
            instr_addr.red(),
            offset.cyan(),
            instr_str.green()
        );
    }
}
