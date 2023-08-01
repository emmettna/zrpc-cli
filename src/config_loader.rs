use config::*;

pub fn config() -> Config {
    config_from_path("config.json")
}

pub fn config_from_path(s: &str) -> Config {
    Config::builder()
        .add_source(File::with_name(s))
        .build()
        .unwrap()
}