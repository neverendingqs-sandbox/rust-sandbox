pub fn test() -> u8 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_0() {
        assert_eq!(0, test());
    }
}
