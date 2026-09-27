pub fn print_twice(message: String) -> (String, String) {
    let borrowed = message.as_str();
    (borrowed.to_string(), borrowed.to_string())
}

#[cfg(test)]
mod tests {
    use super::print_twice;

    #[test]
    fn returns_same_message_twice() {
        let value = String::from("hello");
        let (first, second) = print_twice(value.clone());

        assert_eq!(first, "hello");
        assert_eq!(second, "hello");
        assert_eq!(value, "hello");
    }
}
