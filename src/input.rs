use crate::display;
use crate::error::InputError;
use colored::Colorize;
use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

pub fn read_line() -> Result<String, InputError> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(InputError::ReadFailed)?;

    // Trim the right side of the buffer to remove the newline character
    Ok(buffer.trim_end().to_string())
}

pub fn read_range<T>(min: T, max: T) -> Result<T, InputError>
where
    T: PartialOrd + FromStr + Debug + Display + Copy,
    <T as FromStr>::Err: Debug,
{
    loop {
        let input = read_line()?
            .parse()
            .map_err(|e| InputError::ParseFailed(format!("{:?}", e)))?;

        if input >= min && input <= max {
            return Ok(input);
        } else {
            display::input(format!("input must be between {}-{}", min, max));
        }
    }
}

pub fn select_from_list<T>(items: &[T]) -> Result<usize, InputError>
where
    T: AsRef<str>,
{
    if items.is_empty() {
        return Err(InputError::NoItemsProvided);
    }

    display::input("enter the number next to your choice:");

    for (i, item) in items.iter().enumerate() {
        display::input(format!("{} [{}]", 1 + i, item.as_ref().blue()));
    }

    let index = read_range(1, items.len() + 1)? - 1;

    Ok(index)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Answer {
    Yes,
    No,
}

impl Into<bool> for Answer {
    fn into(self) -> bool {
        match self {
            Answer::Yes => true,
            Answer::No => false,
        }
    }
}

pub fn read_yn(default: Answer) -> Result<Answer, InputError> {
    match read_line()?.as_ref() {
        "y" | "Y" => Ok(Answer::Yes),
        "n" | "N" => Ok(Answer::No),
        _ => Ok(default),
    }
}
