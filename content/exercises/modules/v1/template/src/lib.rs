pub mod text_tools {
    fn count_words(text: &str) -> usize {
        let _ = text;
        0
    }

    pub fn word_count(text: &str) -> usize {
        count_words(text)
    }
}

use crate::text_tools::word_count;

pub fn word_summary(text: &str) -> String {
    let _ = text;
    String::new()
}

#[cfg(test)]
mod tests {
    use super::{text_tools, word_summary};

    #[test]
    fn public_module_api_counts_words() {
        assert_eq!(text_tools::word_count("Rust makes systems programming fun"), 5);
    }

    #[test]
    fn imported_api_summarizes_empty_and_nonempty_text() {
        assert_eq!(word_summary(""), "Words: 0");
        assert_eq!(word_summary("hello Rust"), "Words: 2");
    }
}