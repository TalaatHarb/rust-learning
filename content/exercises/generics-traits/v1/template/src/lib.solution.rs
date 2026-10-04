pub trait Ranked {
    fn rank(&self) -> i64;
}

impl Ranked for i32 {
    fn rank(&self) -> i64 {
        i64::from(*self)
    }
}

impl Ranked for u32 {
    fn rank(&self) -> i64 {
        i64::from(*self)
    }
}

pub fn largest<T: Ranked + Copy>(values: &[T]) -> Option<T> {
    let mut largest = *values.first()?;
    for value in values.iter().copied().skip(1) {
        if value.rank() > largest.rank() {
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
        assert_eq!(largest(&[1_u32, 4, 3]), Some(4));
    }

    #[test]
    fn handles_single_and_empty_inputs() {
        assert_eq!(largest(&[7]), Some(7));
        assert_eq!(largest::<u32>(&[]), None);
    }
}
