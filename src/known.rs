/// A type who impls Known can return a certain value just by itself.
pub trait Known {
    fn get_value(&self) -> f64;
}

impl Known for f64 {
    fn get_value(&self) -> f64 {
        *self
    }
}

impl Known for i32 {
    fn get_value(&self) -> f64 {
        *self as f64
    }
}
