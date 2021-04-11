pub fn add_one(x: i32) -> i32 {
    return x + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(3, add_one(2))
    }
}
