pub fn nth_root(x: f64, n: i32) -> Option<f64> {
    if x < 0.0 && n % 2 == 0 {
        return None;
    }
    if n == 0 {
        return Some(1.0);  // I don`t care if someone try to use 0^0
    }

    let result = f64::powf(x, 1.0 / n as f64);
    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::root::nth_root;
    use crate::round::custom_round;

    #[test]
    fn root_test() {
        let a = nth_root(2.0, 2).unwrap();
        assert_eq!(custom_round(a, 3).unwrap(), 1.414);
    }
}
