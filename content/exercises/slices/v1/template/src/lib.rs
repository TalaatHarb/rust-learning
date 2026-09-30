pub fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn returns_prefix_before_space() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn returns_full_text_without_spaces() {
        assert_eq!(first_word("rust"), "rust");
    }
}
