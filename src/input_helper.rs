// input helper

use std::io::{self, Write};
use std::str::FromStr;

pub fn input<T>(prompt: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    print!("{}", prompt);
    io::stdout().flush().expect("Failed");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read the line");

    buffer.trim().parse::<T>().expect("Failed to parse")
}
