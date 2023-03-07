pub use core::fmt;
pub use std::error::Error;
pub use std::num::ParseIntError;
pub use std::str::FromStr;

pub fn from_hex(input: &str) -> Result<i32, ParseIntError> {
    let without_prefix = input.trim_start_matches("0x");
    i32::from_str_radix(without_prefix, 16)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_hex() {
        println!("{:?}", from_hex("0x00200002"));
    }
}
