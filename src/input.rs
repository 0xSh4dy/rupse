use std::io::{self, Write};

pub fn input_prompt(display_str: String) -> io::Result<(String)> {
    print!("{}> ", display_str);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
