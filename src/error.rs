// TODO: Refactor

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfitError {
    #[error("mising config dir")]
    MissingConfigDir,

    #[cfg(feature = "json")]
    #[error("{0}")]
    SerializeJson(#[source] serde_json::Error),
    #[cfg(feature = "ron")]
    #[error("{0}")]
    SerializeRon(#[source] ron::Error),
    #[cfg(feature = "toml")]
    #[error("{0}")]
    SerializeToml(#[source] toml::ser::Error),
    #[cfg(feature = "yaml")]
    #[error("{0}")]
    SerializeYaml(#[source] serde_yaml::Error),

    #[cfg(feature = "json")]
    #[error("{0}")]
    DeserializeJson(#[source] serde_json::Error),
    #[cfg(feature = "ron")]
    #[error("{0}")]
    DeserializeRon(#[source] ron::de::SpannedError),
    #[cfg(feature = "toml")]
    #[error("{0}")]
    DeserializeToml(#[source] toml::de::Error),
    #[cfg(feature = "yaml")]
    #[error("{0}")]
    DeserializeYaml(#[source] serde_yaml::Error),

    #[error("{0}")]
    IoReadFile(#[source] std::io::Error),
    #[error("{0}")]
    IoCeateDir(#[source] std::io::Error),
    #[error("{0}")]
    IoWriteFile(#[source] std::io::Error),
}
