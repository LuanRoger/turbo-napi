/// Adds 100 to the provided value.
pub fn plus_100(input: u32) -> u32 {
    input + 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_100() {
        assert_eq!(plus_100(23), 123);
    }
}
