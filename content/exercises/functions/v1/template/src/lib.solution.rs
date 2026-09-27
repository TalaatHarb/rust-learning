pub fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}

#[cfg(test)]
mod tests {
    use super::rectangle_area;

    #[test]
    fn calculates_expected_area() {
        assert_eq!(rectangle_area(3, 4), 12);
    }

    #[test]
    fn allows_zero_dimension() {
        assert_eq!(rectangle_area(0, 7), 0);
    }
}
