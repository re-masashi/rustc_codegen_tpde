use std::collections::HashMap;
use std::sync::LazyLock;
use std::{fs, process};

static CONFIG_FILE: LazyLock<HashMap<String, Option<String>>> = LazyLock::new(|| {
    let mut config = fs::read_to_string("config.txt").unwrap();
    config.push_str(&fs::read_to_string("config.user.txt").unwrap_or_default());
    config
        .lines()
        .map(|line| if let Some((line, _comment)) = line.split_once('#') { line } else { line })
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            if let Some((key, val)) = line.split_once('=') {
                (key.trim().to_owned(), Some(val.trim().to_owned()))
            } else {
                (line.to_owned(), None)
            }
        })
        .collect()
});

#[allow(unused)]
pub(crate) fn get_value(key: &str) -> Option<&str> {
    CONFIG_FILE.get(key)?.as_deref()
}

#[allow(unused)]
pub(crate) fn get_bool(name: &str) -> bool {
    if let Some(value) = CONFIG_FILE.get(name) {
        if value.is_some() {
            eprintln!("Boolean config `{}` has a value", name);
            process::exit(1);
        }
        true
    } else {
        false
    }
}
