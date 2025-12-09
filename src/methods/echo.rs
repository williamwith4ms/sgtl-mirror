/// Returns the input string unchanged.
pub fn echo(input: &str) -> &str {
	input
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_echo() {
		let input = "Hello, world!";
		assert_eq!(echo(input), input);
	}
}
