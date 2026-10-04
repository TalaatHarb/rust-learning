pub fn parse_count(input: Option<&str>) -> Result<u32, String> {
    let text = input.ok_or_else(|| String::from("missing input"))?;
    text.parse::<u32>().map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_count;

    #[test]
    fn parses_valid_counts() {
        assert_eq!(parse_count(Some("42")), Ok(42));
    }

    #[test]
    fn reports_missing_and_malformed_input() {
        assert_eq!(parse_count(None), Err(String::from("missing input")));
        assert!(parse_count(Some("nope")).is_err());
    }
}
