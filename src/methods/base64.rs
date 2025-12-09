use base64::prelude::*;

pub fn base64_encode(input: &str) -> String {
    BASE64_STANDARD.encode(input)
}

pub fn base64_decode(input: &str) -> Result<String, base64::DecodeError> {
    let decoded_bytes: Vec<u8> = BASE64_STANDARD.decode(input)?;
    let decoded_str: String = String::from_utf8_lossy(&decoded_bytes).to_string();
    Ok(decoded_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base64_encode() {
        let input: &str = "Hello, world!";
        let expected: &str = "SGVsbG8sIHdvcmxkIQ==";
        assert_eq!(base64_encode(input), expected);
    }
    #[test]
    fn test_base64_decode() {
        let input: &str = "SGVsbG8sIHdvcmxkIQ==";
        let expected: &str = "Hello, world!";
        assert_eq!(base64_decode(input).unwrap(), expected);
    }
}