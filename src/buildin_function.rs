use crate::function::Function;

/// a / b
pub(crate) fn div(a: f64, b: f64) -> Option<f64> {
    return if b == 0.0 {
        None
    } else {
        Some(a / b)
    };
}

/// By Simpson`s method of calculate an integration
/// \int_a^b{}f(x)\d{}x \approx \frac{b - a}{6}\left(f(a) + 4f\left(\frac{a + b}{2}\right) + f(b)\right)
pub(crate) fn int(lo: f64, up: f64, po: Vec<f64>) -> Option<f64> {
    if po.len() < 3 {
        return None;
    }

    let fa = (up - lo) / 6.0;
    let nu = po[0] + 4.0 * po[1] + po[2];

    Some(fa * nu)
}

pub fn int_auto_filler(fun: fn(f64) -> f64, lo: f64, up: f64) -> Vec<f64> {
    vec![fun(lo), fun((lo + up) / 2.0), fun(up)]
}

#[cfg(test)]
mod tests {
    use math::round::custom_round;
    use crate::buildin_function::{div, int, int_auto_filler};

    #[test]
    fn div_test() {
        assert_eq!(div(1.0, 2.0).unwrap(), 0.5);
        assert!(div(1.0, 0.0).is_none());
    }

    #[test]
    fn int_test() {
        let fun = |x: f64| x * x;
        let lo = 1.0;
        let up = 2.0;
        let nu = int_auto_filler(fun, lo, up);
        let re = custom_round(int(lo, up, nu).unwrap(), 3).unwrap();

        assert_eq!(re, 2.333);
    }
}
