use crate::buildin_function::*;

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

type Container = Vec<Box<dyn Known>>;
type CalcContainer = fn(Container, Container) -> Option<f64>;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub calc: CalcContainer,
}

impl Function {
    pub fn new(name: &str, calc: CalcContainer) -> Self {
        Function {
            name: name.to_string(),
            calc,
        }
    }
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
pub fn get_function<'a>(name: &String) -> Result<&'a Function, String> {
    if let Some(fun) =
        BUILD_IN_FUNCTION.iter().find(|f| f.name == *name)
    {
        return Ok(fun);
    } else {
        unsafe {
            if let Some(fun) =
                EXTERN_FUNCTION.iter().find(|f| f.name == *name)
            {
                return Ok(fun);
            }
        }
    }

    Err(format!("Can`t get the function: {name}"))
}

#[cfg(test)]
mod tests {
    use crate::function::{Function, get_function, register_extern_function};

    #[test]
    fn function_test() {
        let frac = Function {
            name: "frac".to_string(),
            calc: |_o, r| {
                Some(r[0].get_value().unwrap() / r[1].get_value().unwrap())
            },
        };

        let a = (frac.calc)(vec![], vec![Box::new(1.0), Box::new(2.0)]);
        assert_eq!(a.unwrap(), 0.5);

        let a = (frac.calc)(vec![], vec![Box::new(3.0), Box::new(2.0)]);
        assert_eq!(a.unwrap(), 1.5);
    }

    #[test]
    fn get_function_test() {
        let fun = get_function(&"frac".to_string()).unwrap();
        assert_eq!(fun.name, "frac");
        assert_eq!((fun.calc)(vec![], vec![Box::new(1.0), Box::new(2.0)]).unwrap(), 0.5);
    }

    #[test]
    fn register_function_test() {
        let re = Function::new("double", |_o, r| {
            Some(r[0].get_value().unwrap() * 2.0)
        });

        register_extern_function(re).expect("Register function failed!");
        let fun = get_function(&"double".to_string()).unwrap();
        assert_eq!((fun.calc)(vec![], vec![Box::new(10.0)]).unwrap(), 20.0);
    }
}
