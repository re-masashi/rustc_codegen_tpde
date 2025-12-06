use std::collections::HashSet;
use std::fs;
use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub keep_sysroot: bool,

    pub skip_tests: HashSet<String>,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config = fs::read_to_string("config.toml").unwrap();
    toml::de::from_str(&config).unwrap()
});

#[allow(unused)]
pub(crate) fn get_config() -> &'static Config {
    &CONFIG
}
