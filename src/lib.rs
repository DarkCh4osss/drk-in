use std::io;
use std::io::Write;

pub fn input(user_message: &str) -> io::Result<String> {
    
    print!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

pub fn parse_to_int(user_string: &str) -> i32 {
    
    let buffer: i32 = match user_string.parse::<i32>() {
        Ok(buffer) => buffer;
        Err(_e) => -1,
    };

    buffer
}