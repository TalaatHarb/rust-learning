pub fn summarize_reading(reading: (u8, bool), samples: [i16; 3]) -> (u8, bool, i32) {
    let (level, active) = reading;
    let total: i32 = samples.iter().map(|sample| i32::from(*sample)).sum();
    (level, active, total)
}

#[cfg(test)]
mod tests {
    use super::summarize_reading;

    #[test]
    fn summarizes_tuple_and_array_values() {
        assert_eq!(summarize_reading((7, true), [4, 8, 12]), (7, true, 24));
    }

    #[test]
    fn handles_zero_values() {
        assert_eq!(summarize_reading((0, false), [0, 0, 0]), (0, false, 0));
    }
}
