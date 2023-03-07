use super::errors::ParseError;
use std::num::ParseIntError;

pub fn from_hex(input: &str) -> Result<i32, ParseIntError> {
    let without_prefix = input.trim_start_matches("0x");
    i32::from_str_radix(without_prefix, 16)
}

pub fn process_event_reply<'a>(
    reply: &'a str,
    event: &str,
    arg_num: usize,
) -> Result<Vec<&'a str>, ParseError> {
    let split: Vec<_> = reply.trim().split(' ').collect();

    if split.len() == 0 {
        return Err(ParseError::InsufficientData);
    }

    if split[0] != event {
        return Err(ParseError::InvalidEvent);
    }

    if split.len() < arg_num + 1 {
        return Err(ParseError::InsufficientData);
    }

    Ok(split)
}

pub fn get_event_type(reply: &str) -> Result<&str, ParseError> {
    let split: Vec<_> = reply.trim().split(' ').collect();

    if split.len() == 0 {
        return Err(ParseError::InsufficientData);
    }

    Ok(split[0])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_hex() {
        println!("{:?}", from_hex("0x00200002"));
    }
}
