pub fn classify_number(value: i32) -> &'static str {
    if value >= 0 {
        "positive"
    } else {
        "negative"
    }
}

#[cfg(test)]
mod tests {
    use super::classify_number;

    #[test]
    fn classifies_negative_numbers() {
        assert_eq!(classify_number(-4), "negative");
    }

    #[test]
    fn classifies_zero_separately() {
        assert_eq!(classify_number(0), "zero");
    }

    #[test]
    fn classifies_positive_numbers() {
        assert_eq!(classify_number(7), "positive");
    }
}
