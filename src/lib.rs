use std::io;
use std::io::Write;

pub fn input(user_message: &str) -> io::Result<String> {
    
    print!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}