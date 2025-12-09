use sha2::{Sha256, Digest};

pub fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn sha512_hash(input: &str) -> String {
    let mut hasher = sha2::Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn sha384_hash(input: &str) -> String {
    let mut hasher = sha2::Sha384::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn sha224_hash(input: &str) -> String {
    let mut hasher = sha2::Sha224::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn sha512_256_hash(input: &str) -> String {
    let mut hasher = sha2::Sha512_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sha256_hash() {
        let input: &str = "Hello, world!";
        let expected: &str = "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3";
        assert_eq!(sha256_hash(input), expected);
    }

    #[test]
    fn test_sha512_hash() {
        let input: &str = "Hello, world!";
        let expected: &str = "c1527cd893c124773d811911970c8fe6e857d6df5dc9226bd8a160614c0cd963a4ddea2b94bb7d36021ef9d865d5cea294a82dd49a0bb269f51f6e7a57f79421";
        assert_eq!(sha512_hash(input), expected);
    }

    #[test]
    fn test_sha384_hash() {
        let input: &str = "Hello, world!";
        let expected: &str = "55bc556b0d2fe0fce582ba5fe07baafff035653638c7ac0d5494c2a64c0bea1cc57331c7c12a45cdbca7f4c34a089eeb";
        assert_eq!(sha384_hash(input), expected);   
    }

    #[test]
    fn test_sha224_hash() {
        let input: &str = "Hello, world!";
        let expected: &str = "8552d8b7a7dc5476cb9e25dee69a8091290764b7f2a64fe6e78e9568";
        assert_eq!(sha224_hash(input), expected);
    }

    #[test]
    fn test_sha512_256_hash() {
        let input: &str = "Hello, world!";
        let expected: &str = "330c723f25267587db0b9f493463e017011239169cb57a6db216c63774367115";
        assert_eq!(sha512_256_hash(input), expected);
    }
}