use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn input<T: FromStr>(user_message: &str) -> Option<T> {
    
    print!("{}", user_message);
    io::stdout().flush().ok()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).ok()?;

    Some(buffer.trim().parse().ok()?)
}

#[macro_export]
macro_rules! tinput {
    ($f:ty $(,$t:ty)*) => {
        (input::<$f>("") $(, input::<$t>(""))*)
    }
}

pub fn parse_to_int(user_string: &str) -> i32 {
    
    let buffer: i32 = match user_string.parse::<i32>() {
        Ok(buffer) => buffer,
        Err(_e) => -1,
    };

    buffer
}