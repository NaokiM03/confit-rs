use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use std::{fs, path::PathBuf};

mod config_dir;
mod extension;

use config_dir::config_dir;
pub use extension::Extension;

#[cfg(all(
    not(feature = "json"),
    not(feature = "ron"),
    not(feature = "toml"),
    not(feature = "yaml")
))]
compile_error!(
    "At least one feature must be enabled. \
Please enable `json`, `ron`, `toml` or `yaml`."
);

#[derive(Debug, Error)]
pub enum ConfitError {
    #[error("mising config dir")]
    MissingConfigDir,

    #[cfg(feature = "json")]
    #[error("{0}")]
    FailedToSerializeJson(#[source] serde_json::Error),
    #[cfg(feature = "ron")]
    #[error("{0}")]
    FailedToSerializeRon(#[source] ron::Error),
    #[cfg(feature = "toml")]
    #[error("{0}")]
    FailedToSerializeToml(#[source] toml::ser::Error),
    #[cfg(feature = "yaml")]
    #[error("{0}")]
    FailedToSerializeYaml(#[source] serde_yaml::Error),

    #[cfg(feature = "json")]
    #[error("{0}")]
    FailedToDeserializeJson(#[source] serde_json::Error),
    #[cfg(feature = "ron")]
    #[error("{0}")]
    FailedToDeserializeRon(#[source] ron::de::SpannedError),
    #[cfg(feature = "toml")]
    #[error("{0}")]
    FailedToDeserializeToml(#[source] toml::de::Error),
    #[cfg(feature = "yaml")]
    #[error("{0}")]
    FailedToDeserializeYaml(#[source] serde_yaml::Error),

    #[error("{0}")]
    IoReadFile(#[source] std::io::Error),
    #[error("{0}")]
    IoCeateDir(#[source] std::io::Error),
    #[error("{0}")]
    IoWriteFile(#[source] std::io::Error),
}

fn config_file(
    app_name: &str,
    file_name: &str,
    extension: &Extension,
) -> Result<PathBuf, ConfitError> {
    let file_name = format!("{file_name}.{extension}");
    config_dir()
        .ok_or(ConfitError::MissingConfigDir)
        .map(|dir| dir.join(app_name).join(file_name))
}

fn serialize<T: Serialize>(config: &T, extension: Extension) -> Result<String, ConfitError> {
    match extension {
        #[cfg(feature = "json")]
        Extension::Json => {
            serde_json::to_string_pretty(&config).map_err(ConfitError::FailedToSerializeJson)
        }

        #[cfg(feature = "ron")]
        Extension::Ron => {
            let option = ron::ser::PrettyConfig::default().new_line("\n".to_owned());
            ron::ser::to_string_pretty(&config, option)
        }
        .map_err(ConfitError::FailedToSerializeRon),

        #[cfg(feature = "toml")]
        Extension::Toml => {
            toml::to_string_pretty(&config).map_err(ConfitError::FailedToSerializeToml)
        }

        #[cfg(feature = "yaml")]
        Extension::Yaml => {
            serde_yaml::to_string(&config).map_err(ConfitError::FailedToSerializeYaml)
        }
    }
}

fn deserialize<T: DeserializeOwned>(config: &str, extension: Extension) -> Result<T, ConfitError> {
    match extension {
        #[cfg(feature = "json")]
        Extension::Json => {
            serde_json::from_str(config).map_err(ConfitError::FailedToDeserializeJson)
        }

        #[cfg(feature = "ron")]
        Extension::Ron => ron::from_str(config).map_err(ConfitError::FailedToDeserializeRon),

        #[cfg(feature = "toml")]
        Extension::Toml => toml::from_str(config).map_err(ConfitError::FailedToDeserializeToml),

        #[cfg(feature = "yaml")]
        Extension::Yaml => {
            serde_yaml::from_str(config).map_err(ConfitError::FailedToDeserializeYaml)
        }
    }
}

pub fn load_or_init<T: Serialize + DeserializeOwned + Default>(
    app_name: &str,
    file_name: &str,
    extension: Extension,
) -> Result<T, ConfitError> {
    let path = config_file(app_name, file_name, &extension)?;

    if path.exists() {
        let config = fs::read_to_string(path).map_err(ConfitError::IoReadFile)?;
        return deserialize(&config, extension);
    }

    if let Some(p) = path.parent() {
        fs::create_dir_all(p).map_err(ConfitError::IoCeateDir)?;
    }

    let config = T::default();
    {
        let contents = serialize(&config, extension)?;
        fs::write(path, contents).map_err(ConfitError::IoWriteFile)?;
    }

    Ok(config)
}

pub fn store<T: Serialize>(
    app_name: &str,
    file_name: &str,
    extension: Extension,
    config: T,
) -> Result<(), ConfitError> {
    let path = config_file(app_name, file_name, &extension)?;
    if !path.exists() {
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).map_err(ConfitError::IoCeateDir)?;
        }
    }

    let config = serialize(&config, extension)?;
    fs::write(path, config).map_err(ConfitError::IoWriteFile)?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use serde::{Deserialize, Serialize};

//     use super::*;

//     const APP_NAME: &str = "Confit";
//     const FILE_NAME: &str = "settings";

//     #[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
//     struct Settings {
//         a: String,
//         b: u64,
//         c: bool,
//         d: Vec<String>,
//     }

//     impl Settings {
//         fn create_test_data() -> Self {
//             Settings {
//                 a: "str".to_owned(),
//                 b: 42,
//                 c: true,
//                 d: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
//             }
//         }
//     }

//     #[test]
//     fn test_load_or_init() {
//         let res: Settings = load_or_init(APP_NAME, FILE_NAME, Extension::Json).unwrap();
//         assert_eq!(res, Settings::default());
//     }

//     #[test]
//     fn test_store() {
//         let settings = Settings::create_test_data();

//         let _ = store(APP_NAME, FILE_NAME, Extension::Json, settings).unwrap();

//         let res: Settings = load_or_init(APP_NAME, FILE_NAME, Extension::Json).unwrap();
//         assert_eq!(res.a, "str");
//         assert_eq!(res.b, 42);
//         assert_eq!(res.c, true);
//         assert_eq!(
//             res.d,
//             vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]
//         );

//         // reset
//         let path = config_file(APP_NAME, FILE_NAME, &Extension::Json).unwrap();
//         fs::remove_file(path).unwrap();
//     }
// }
