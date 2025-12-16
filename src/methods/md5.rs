use md5::compute;

pub fn md5_hash(input: &str) -> String {
    let digest = compute(input);
    format!("{:x}", digest)
}