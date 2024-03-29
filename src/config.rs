use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub high_accuracy: bool,
}

fn get_config() -> String {
    // let path = get_config_path();
    // let mut buffer = Vec::new();
    // fs::File::open(path).unwrap_or_else(|_| panic!("Can not read CONFIG file"))
    //     .read_to_end(&mut buffer).unwrap();
    //
    // match String::from_utf8(buffer) {
    //     Ok(s) => s.clone(),
    //     Err(_) => {
    //         panic!("Can not format CONFIG content to UTF-8");
    //     }
    // }
    // TODO!
    "high_accuracy = false".to_string()
}

lazy_static! {
    pub static ref CONFIG: Config = {
        toml::from_str(get_config().as_str()).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use crate::config::get_config;

    #[test]
    fn read_config_test() {
        println!("{}", get_config());
    }
}
