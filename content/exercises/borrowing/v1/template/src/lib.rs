pub fn longer_label(left: &str, right: &str) -> String {
    if left.len() >= right.len() {
        right.to_string()
    } else {
        left.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::longer_label;

    #[test]
    fn returns_longer_input() {
        let first = String::from("ownership");
        let second = String::from("borrow");

        assert_eq!(longer_label(&first, &second), "ownership");
        assert_eq!(first, "ownership");
        assert_eq!(second, "borrow");
    }

    #[test]
    fn keeps_right_value_when_it_is_longer() {
        let first = String::from("rust");
        let second = String::from("compiler");

        assert_eq!(longer_label(&first, &second), "compiler");
        assert_eq!(first, "rust");
        assert_eq!(second, "compiler");
    }
}
