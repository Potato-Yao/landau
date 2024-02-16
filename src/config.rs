use std::fs;
use std::io::Read;
use lazy_static::lazy_static;
use serde::Deserialize;
use crate::get_config_path;

#[derive(Deserialize)]
pub struct Config {
    pub high_accuracy: bool,
}

fn get_config() -> String {
    let path = get_config_path();
    let mut buffer = Vec::new();
    fs::File::open(path).unwrap_or_else(|_| panic!("Can not read CONFIG file"))
        .read_to_end(&mut buffer).unwrap();

    match String::from_utf8(buffer) {
        Ok(s) => unsafe {
            s.clone()
        }
        Err(_) => {
            panic!("Can not format CONFIG content to UTF-8");
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        toml::from_str(get_config().as_str()).unwrap()
    };
}
