pub fn print_twice(_message: String) -> (String, String) {
    todo!("Borrow message so ownership is preserved")
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
