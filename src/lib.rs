use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};

use std::{
    fs,
    path::{Path, PathBuf},
};

mod config_dir;
mod extension;

use config_dir::config_dir;
pub use extension::Extension;

fn app_config_dir(app_name: &str) -> Result<PathBuf> {
    let app_config_dir = config_dir().context("missing config dir")?.join(app_name);
    Ok(app_config_dir)
}

fn config_file(app_config_dir: &Path, file_name: &str, extension: Extension) -> Result<PathBuf> {
    let config_file = format!("{file_name}.{extension}");
    let config_file = app_config_dir.join(config_file);

    Ok(config_file)
}

pub fn load_or_init<T: Serialize + DeserializeOwned + Default>(
    app_name: &str,
    file_name: &str,
    extension: Extension,
) -> Result<T> {
    let app_config_dir = app_config_dir(app_name)?;
    let config_file = config_file(&app_config_dir, file_name, extension)?;

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

pub fn store<T: Serialize>(
    app_name: &str,
    file_name: &str,
    extension: Extension,
    config: T,
) -> Result<()> {
    let app_config_dir = app_config_dir(app_name)?;
    let config_file = config_file(&app_config_dir, file_name, extension)?;

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
//     struct Config {
//         foo: String,
//     }

//     #[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
//     struct Theme {
//         bar: String,
//     }

//     const APP_NAME: &str = "confit";

//     #[test]
//     fn test_load_or_init() -> Result<()> {
//         let expect = Config {
//             foo: String::default(),
//         };
//         let actual: Config = load_or_init(APP_NAME, "config", Extension::Json)?;
//         assert_eq!(expect, actual);

//         let expect = Theme {
//             bar: String::default(),
//         };
//         let actual: Theme = load_or_init(APP_NAME, "theme", Extension::Json)?;
//         assert_eq!(expect, actual);

//         // {
//         //     let app_config_dir = app_config_dir(APP_NAME)?;

//         //     let config = config_file(&app_config_dir, "config", Extension::Json)?;
//         //     let theme = config_file(&app_config_dir, "theme", Extension::Json)?;

//         //     assert!(fs::remove_file(config).is_ok());
//         //     assert!(fs::remove_file(theme).is_ok());
//         // }

//         Ok(())
//     }

//     #[test]
//     fn test_store() -> Result<()> {
//         let config = Config {
//             foo: String::default(),
//         };

//         store(APP_NAME, "config", Extension::Json, &config)?;

//         let expect = config;
//         let actual: Config = load_or_init(APP_NAME, "config", Extension::Json)?;
//         assert_eq!(expect, actual);

//         let theme = Theme {
//             bar: String::default(),
//         };

//         store(APP_NAME, "theme", Extension::Json, &theme)?;

//         let expect = theme;
//         let actual: Theme = load_or_init(APP_NAME, "theme", Extension::Json)?;
//         assert_eq!(expect, actual);

//         // {
//         //     let app_config_dir = app_config_dir(APP_NAME)?;

//         //     let config = config_file(&app_config_dir, "config", Extension::Json)?;
//         //     let theme = config_file(&app_config_dir, "theme", Extension::Json)?;

//         //     assert!(fs::remove_file(config).is_ok());
//         //     assert!(fs::remove_file(theme).is_ok());
//         // }

//         Ok(())
//     }
// }
