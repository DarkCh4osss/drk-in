use std::io;
use std::io::Write;
use std::str::FromStr;
// use std::io::prelude::*;

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

// pub fn pause() {
//   let mut stdin = io::stdin();
//   let mut stdout = io::stdout();

//   write!(stdout, "Press any key to continue...").unwrap();
//   stdout.flush().unwrap();

//   let _ = stdin.read(&mut [0u8]).unwrap();
// }

#[macro_export]
macro_rules! system {
    {clear || cls} => (print!("{esc}[2J{esc}[1;1H", esc = 27 as char));
    {pause} => (let mut stdin = io::stdin();
    let mut stdout = io::stdout();
  
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
  
    let _ = stdin.read(&mut [0u8]).unwrap(););
}

pub fn parse_to_int(user_string: &str) -> i32 {
    
    let buffer: i32 = match user_string.parse::<i32>() {
        Ok(buffer) => buffer,
        Err(_e) => -1,
    };

    buffer
}