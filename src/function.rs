use lazy_static::lazy_static;
use math::root::nth_root;

/// An type who impls Known can return a certain value just by itself.
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

type Container = Vec<Box<dyn Known>>;

#[derive(Debug)]
pub struct Function {
    name: String,
    calc: fn(Container) -> Option<f64>,
}

impl Function {
    pub fn new(name: &str, calc: fn(Container) -> Option<f64>) -> Self {
        Function {
            name: name.to_string(),
            calc,
        }
    }
}

/// a / b
fn div(a: f64, b: f64) -> Option<f64> {
    return if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

static mut EXTERN_FUNCTION: Vec<Function> = Vec::new();

lazy_static! {
    static ref BUILD_IN_FUNCTION: Vec<Function> = {
        let mut table = Vec::new();
        table.push(Function::new("frac", |v| {
            div(v[0].get_value().unwrap(), v[1].get_value().unwrap())
        }));
        table.push(Function::new("sqrt", |v| {
            nth_root(v[1].get_value().unwrap(), v[0].get_value().unwrap() as i32)
        }));

        table
    };
}

pub fn register_extern_function(fun: Function) -> Result<(), String> {
    unsafe {
        EXTERN_FUNCTION.push(fun);
    }
    Ok(())
}

/// build-in functions take the priority,
/// so if there`s an extern function which has a same name as a build-in function,
/// the extern function will never be gotten
pub fn get_function<'a>(name: String) -> Option<&'a Function> {
    if let Some(fun) =
        BUILD_IN_FUNCTION.iter().find(|f| f.name == name)
    {
        return Some(fun);
    } else {
        unsafe {
            if let Some(fun) =
                EXTERN_FUNCTION.iter().find(|f| f.name == name)
            {
                return Some(fun);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::function::{Known, Function, get_function, register_extern_function};

    #[test]
    fn function_test() {
        let frac = Function {
            name: "frac".to_string(),
            calc: |v| {
                Some(v[0].get_value().unwrap() / v[1].get_value().unwrap())
            },
        };

        let a = (frac.calc)(vec![Box::new(1.0), Box::new(2.0)]);
        assert_eq!(a.unwrap(), 0.5);

        let a = (frac.calc)(vec![Box::new(3.0), Box::new(2.0)]);
        assert_eq!(a.unwrap(), 1.5);
    }

    #[test]
    fn get_function_test() {
        let fun = get_function("frac".to_string()).unwrap();
        assert_eq!(fun.name, "frac");
        assert_eq!((fun.calc)(vec![Box::new(1.0), Box::new(2.0)]).unwrap(), 0.5);
    }

    #[test]
    fn register_function_test() {
        let re = Function::new("double", |v| {
            Some(v[0].get_value().unwrap() * 2.0)
        });

        register_extern_function(re).expect("Register function failed!");
        let fun = get_function("double".to_string()).unwrap();
        assert_eq!((fun.calc)(vec![Box::new(10.0)]).unwrap(), 20.0);
    }
}
