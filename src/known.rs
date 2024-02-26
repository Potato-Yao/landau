use std::ops::Deref;

/// A type who impls Known can return a certain value just by itself.
pub trait Known {
    fn get_value(&self) -> Option<f64>;
}

impl Known for f64 {
    fn get_value(&self) -> Option<f64> {
        Some(*self)
    }
}

impl Known for i32 {
    fn get_value(&self) -> Option<f64> {
        Some(*self as f64)
    }
}

impl Deref for dyn Known {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        
    }
}
