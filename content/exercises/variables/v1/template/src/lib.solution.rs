pub fn increment_counter() -> i32 {
    let mut counter = 0;
    counter += 1;
    counter
}

#[cfg(test)]
mod tests {
    use super::increment_counter;

    #[test]
    fn increments_from_zero_to_one() {
        assert_eq!(increment_counter(), 1);
    }
}
