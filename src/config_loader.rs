use config::*;

pub fn config() -> Config {
    config_from_path("/var/zrpc-cli/config.json")
}

pub fn config_from_path(s: &str) -> Config {
    Config::builder()
        .add_source(File::with_name(s))
        .add_source(config::File::from_str(
            r#"
            log_level = "info"
            auto_correction.max_attempt = 5
            "#,
            config::FileFormat::Toml,
        ))
        .build()
        .unwrap_or_else(|_| {
            Config::builder()
                .add_source(config::File::from_str(
                    r#"
                    log_level = "info"
                    auto_correction.max_attempt = 5
                    "#,
                    config::FileFormat::Toml,
                ))
                .build()
                .unwrap()
        })
}
