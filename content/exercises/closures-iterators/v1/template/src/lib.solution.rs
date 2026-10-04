pub fn even_squares(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .filter(|value| **value % 2 == 0)
        .map(|value| value * value)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::even_squares;

    #[test]
    fn filters_and_squares_in_order() {
        assert_eq!(even_squares(&[3, 2, 4, 5, 6]), vec![4, 16, 36]);
    }

    #[test]
    fn returns_an_empty_vector_when_nothing_matches() {
        assert_eq!(even_squares(&[1, 3, 5]), Vec::<i32>::new());
        assert_eq!(even_squares(&[]), Vec::<i32>::new());
    }
}
