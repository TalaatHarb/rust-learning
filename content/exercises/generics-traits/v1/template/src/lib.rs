pub trait Ranked {
    fn rank(&self) -> i64;
}

impl Ranked for i32 {
    fn rank(&self) -> i64 {
        0
    }
}

impl Ranked for u32 {
    fn rank(&self) -> i64 {
        0
    }
}

pub fn largest<T: Ranked + Copy>(values: &[T]) -> Option<T> {
    let _ = values;
    None
}
