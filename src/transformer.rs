use lazy_static::lazy_static;
use regex::Regex;
use crate::known::Known;

lazy_static! {
    // match numbers, such as 1 or 1.1
    static ref PURE_NUMBER: Regex = Regex::new(r"-?\d+(\.\d+)?").unwrap();
}

pub fn strings_to_known(v: &Vec<String>) -> Vec<Box<dyn Known>> {
    v.iter().map(|s| string_to_known(s).unwrap()).collect()
}

pub fn string_to_known(s: &String) -> Option<Box<dyn Known>> {
    #[cfg(debug_assertions)]
    {
        eprintln!("i catch: {s}");
    }
    if PURE_NUMBER.is_match(s) {
        return Some(Box::new(s.parse::<f64>().unwrap()));
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::transformer::string_to_known;

    #[test]
    fn string_to_known_test() {
        let s1 = "-2".to_string();
        let r1 = string_to_known(&s1).unwrap().get_value();
        assert_eq!(r1, -2.0);

        let s2 = "2.1".to_string();
        let r2 = string_to_known(&s2).unwrap().get_value();
        assert_eq!(r2, 2.1);

        let s3 = "-0.110".to_string();
        let r3 = string_to_known(&s3).unwrap().get_value();
        assert_eq!(r3, -0.11);

        let s4 = "01.2".to_string();
        let r4 = string_to_known(&s4).unwrap().get_value();
        assert_eq!(r4, 1.2);
    }
}
