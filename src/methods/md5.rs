use md5::compute;

pub fn md5_hash(input: &str) -> String {
    let digest = compute(input);
    format!("{:x}", digest)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_md5_hash() {
        let input = "Hello, world!";
        let expected = "6cd3556deb0da54bca060b4c39479839";
        assert_eq!(md5_hash(input), expected);
    }
}