use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};

use std::{fs, path::PathBuf};

mod config_dir;

use config_dir::config_dir;

#[cfg(feature = "ext_json")]
const CONFIG_FILE_NAME: &str = "config.json";

pub fn load_or_init<T: Serialize + DeserializeOwned + Default>(app_name: &str) -> Result<T> {
    let app_config_dir = config_dir().context("missing config dir")?.join(app_name);
    let config_file: PathBuf = app_config_dir.join(CONFIG_FILE_NAME);

    let config = if config_file.exists() {
        let config = fs::read_to_string(config_file)?;
        serde_json::from_str(&config)?
    } else {
        fs::create_dir_all(&app_config_dir)?;

        let config = T::default();
        {
            let config = serde_json::to_string(&config)?;
            fs::write(config_file, config)?;
        }

        config
    };

    Ok(config)
}

pub fn store<T: Serialize>(app_name: &str, config: T) -> Result<()> {
    let app_config_dir = config_dir().context("missing config dir")?.join(app_name);
    let config_file: std::path::PathBuf = app_config_dir.join(CONFIG_FILE_NAME);

    let config = serde_json::to_string(&config)?;

    fs::create_dir_all(&app_config_dir)?;
    fs::write(config_file, config)?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use serde::{Deserialize, Serialize};

//     #[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
//     struct FooConfig {
//         bar: String,
//     }

//     const APP_NAME: &str = "confit";

//     #[test]
//     fn test_load_or_init() {
//         let expect = FooConfig {
//             bar: String::default()
//         };
//         let actual: FooConfig = load_or_init(APP_NAME).unwrap();

//         assert_eq!(expect, actual);
//     }

//     #[test]
//     fn test_store() {
//         let config = FooConfig {
//             bar: "baz".to_owned(),
//         };
//         assert!(store(APP_NAME, &config).is_ok());

//         let expect = config;
//         let actual: FooConfig = load_or_init(APP_NAME).unwrap();

//         assert_eq!(expect, actual);
//     }
// }
