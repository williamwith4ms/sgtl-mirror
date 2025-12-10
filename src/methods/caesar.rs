pub fn caesar_encipher(input: &str, shift: i8) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = ((c as u8 - base + (shift as u8)) % 26) + base;
            shifted as char
        } else {
            c
        }
    }).collect()
}

pub fn caesar_decipher(input: &str, shift: i8) -> String {
    caesar_encipher(input, 26 - (shift % 26))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_caesar_encipher() {
        let input: &str = "Hello, World!";
        let shift: i8 = 3;
        let expected: &str = "Khoor, Zruog!";
        assert_eq!(caesar_encipher(input, shift), expected);
    }

    #[test]
    fn test_caesar_decipher() {
        let input: &str = "Khoor, Zruog!";
        let shift: i8 = 3;
        let expected: &str = "Hello, World!";
        assert_eq!(caesar_decipher(input, shift), expected);
    }
}
