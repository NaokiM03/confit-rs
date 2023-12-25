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

fn config_file(app_config_dir: &Path, file_name: &str, extension: &Extension) -> Result<PathBuf> {
    let config_file = format!("{file_name}.{extension}");
    let config_file = app_config_dir.join(config_file);

    Ok(config_file)
}

fn serialize<T: Serialize>(config: &T, extension: &Extension) -> Result<String> {
    let config = match extension {
        #[cfg(feature = "ext_json")]
        Extension::Json => serde_json::to_string_pretty(&config)?,
        #[cfg(feature = "ext_ron")]
        Extension::Ron => ron::ser::to_string_pretty(
            &config,
            ron::ser::PrettyConfig::default().new_line("\n".to_owned()),
        )?,
        #[cfg(feature = "ext_toml")]
        Extension::Toml => toml::to_string_pretty(&config)?,
        #[cfg(feature = "ext_yaml")]
        Extension::Yaml => serde_yaml::to_string(&config)?,
    };
    Ok(config)
}

fn deserialize<T: DeserializeOwned>(config: &str, extension: &Extension) -> Result<T> {
    let config = match extension {
        #[cfg(feature = "ext_json")]
        Extension::Json => serde_json::from_str(&config)?,
        #[cfg(feature = "ext_ron")]
        Extension::Ron => ron::from_str(&config)?,
        #[cfg(feature = "ext_toml")]
        Extension::Toml => toml::from_str(&config)?,
        #[cfg(feature = "ext_yaml")]
        Extension::Yaml => serde_yaml::from_str(&config)?,
    };
    Ok(config)
}

pub fn load_or_init<T: Serialize + DeserializeOwned + Default>(
    app_name: &str,
    file_name: &str,
    extension: &Extension,
) -> Result<T> {
    let app_config_dir = app_config_dir(app_name)?;
    let config_file = config_file(&app_config_dir, file_name, extension)?;

    let config = if config_file.exists() {
        let config = fs::read_to_string(config_file)?;
        deserialize(&config, extension)?
    } else {
        fs::create_dir_all(&app_config_dir)?;

        let config = T::default();
        {
            let config = serialize(&config, extension)?;
            fs::write(config_file, config)?;
        }

        config
    };

    Ok(config)
}

pub fn store<T: Serialize>(
    app_name: &str,
    file_name: &str,
    extension: &Extension,
    config: T,
) -> Result<()> {
    let app_config_dir = app_config_dir(app_name)?;
    let config_file = config_file(&app_config_dir, file_name, extension)?;

    let config = serialize(&config, extension)?;

    fs::create_dir_all(&app_config_dir)?;
    fs::write(config_file, config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
    struct Config {
        a: String,
        b: u64,
        c: bool,
        d: Vec<String>,
    }

    impl Config {
        fn create_test_data() -> Self {
            Config {
                a: "str".to_owned(),
                b: 42,
                c: true,
                d: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
            }
        }
    }

    mod serialize {
        use super::*;

        #[cfg(feature = "ext_json")]
        #[test]
        fn json() -> Result<()> {
            let config = Config::create_test_data();
            let expect = "\
{
  \"a\": \"str\",
  \"b\": 42,
  \"c\": true,
  \"d\": [
    \"foo\",
    \"bar\",
    \"baz\"
  ]
}\
";
            let actual = serialize(&config, &Extension::Json)?;
            assert_eq!(expect, actual);

            Ok(())
        }

        #[cfg(feature = "ext_ron")]
        #[test]
        fn ron() -> Result<()> {
            let config = Config::create_test_data();
            let expect = "\
(
    a: \"str\",
    b: 42,
    c: true,
    d: [
        \"foo\",
        \"bar\",
        \"baz\",
    ],
)\
";
            let actual = serialize(&config, &Extension::Ron)?;
            assert_eq!(expect, actual);

            Ok(())
        }

        #[cfg(feature = "ext_toml")]
        #[test]
        fn toml() -> Result<()> {
            let config = Config::create_test_data();
            let expect = "\
a = \"str\"
b = 42
c = true
d = [
    \"foo\",
    \"bar\",
    \"baz\",
]
";
            let actual = serialize(&config, &Extension::Toml)?;
            assert_eq!(expect, actual);

            Ok(())
        }

        #[cfg(feature = "ext_yaml")]
        #[test]
        fn yaml() -> Result<()> {
            let config = Config::create_test_data();
            let expect = "\
a: str
b: 42
c: true
d:
- foo
- bar
- baz
";
            let actual = serialize(&config, &Extension::Yaml)?;
            assert_eq!(expect, actual);

            Ok(())
        }
    }

    mod deserialize {
        use super::*;

        #[cfg(feature = "ext_json")]
        #[test]
        fn json() -> Result<()> {
            let config = "\
{
  \"a\": \"str\",
  \"b\": 42,
  \"c\": true,
  \"d\": [
    \"foo\",
    \"bar\",
    \"baz\"
  ]
}\
";
            let expect = Config::create_test_data();
            let actual = deserialize(config, &Extension::Json)?;
            assert_eq!(expect, actual);

            Ok(())
        }

        #[cfg(feature = "ext_ron")]
        #[test]
        fn ron() -> Result<()> {
            let config = "\
(
    a: \"str\",
    b: 42,
    c: true,
    d: [
        \"foo\",
        \"bar\",
        \"baz\",
    ],
)\
";
            let expect = Config::create_test_data();
            let actual = deserialize(config, &Extension::Ron)?;
            assert_eq!(expect, actual);

            Ok(())
        }

        #[cfg(feature = "ext_toml")]
        #[test]
        fn toml() -> Result<()> {
            let config = "\
a = \"str\"
b = 42
c = true
d = [
    \"foo\",
    \"bar\",
    \"baz\",
]
";
            let expect = Config::create_test_data();
            let actual = deserialize(config, &Extension::Toml)?;
            assert_eq!(expect, actual);

            Ok(())
        }

        #[cfg(feature = "ext_yaml")]
        #[test]
        fn yaml() -> Result<()> {
            let config = "\
a: str
b: 42
c: true
d:
- foo
- bar
- baz
";
            let expect = Config::create_test_data();
            let actual = deserialize(config, &Extension::Yaml)?;
            assert_eq!(expect, actual);

            Ok(())
        }
    }
}
