pub fn append_rust(message: &mut String) {
    message.push_str(" Rust");
}

#[cfg(test)]
mod tests {
    use super::append_rust;

    #[test]
    fn appends_text_in_place() {
        let mut message = String::from("hello");
        append_rust(&mut message);
        assert_eq!(message, "hello Rust");
    }
}
