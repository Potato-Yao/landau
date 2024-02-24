pub fn custom_approx(u: f64, d: u32) -> Option<f64> {
    if d > 8 {
        return None;
    }
    let s = 10_i32.pow(d) as f64;
    Some((u * s).round() / s)
}

#[cfg(test)]
mod tests {
    use crate::util::approx::custom_approx;

    #[test]
    fn round_test() {
        let a = 1.23456;
        assert_eq!(custom_approx(a, 2), Some(1.23));
    }
}
