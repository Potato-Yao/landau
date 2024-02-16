use std::env;

mod function;
mod exec;
mod buildin_function;
mod transformer;
mod config;

fn get_config_path() -> String {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1);

    match path {
        Some(p) => p.clone(),
        _ => "./config.toml".to_string(),
    }
}
