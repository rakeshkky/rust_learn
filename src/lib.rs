pub mod doubling {
    pub fn double_number(x: i32) -> i32 {
        x * 2
    }
}

#[cfg(test)]
mod tests {
    use crate::doubling::double_number;

    #[test]
    fn test_doubling() {
        assert_eq!(double_number(5), 10);
        assert_eq!(double_number(2), 4);
    }
}
