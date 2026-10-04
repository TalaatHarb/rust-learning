pub fn largest<T: PartialOrd + Copy>(values: &[T]) -> Option<T> {
    let mut largest = *values.first()?;
    for value in values.iter().copied().skip(1) {
        if value > largest {
            largest = value;
        }
    }
    Some(largest)
}

#[cfg(test)]
mod tests {
    use super::largest;

    #[test]
    fn compares_multiple_concrete_types() {
        assert_eq!(largest(&[3, 9, 2]), Some(9));
        assert_eq!(largest(&[1.5_f64, 4.25, 3.0]), Some(4.25));
    }

    #[test]
    fn handles_single_and_empty_inputs() {
        assert_eq!(largest(&[7]), Some(7));
        assert_eq!(largest::<i32>(&[]), None);
    }
}
