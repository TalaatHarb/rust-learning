pub fn print_twice(message: &str) -> (String, String) {
    (message.to_string(), message.to_string())
}

#[cfg(test)]
mod tests {
    use super::print_twice;

    #[test]
    fn returns_same_message_twice() {
        let value = String::from("hello");
        let (first, second) = print_twice(&value);

        assert_eq!(first, "hello");
        assert_eq!(second, "hello");
        assert_eq!(value, "hello");
    }
}
