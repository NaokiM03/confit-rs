use std::fmt::Display;

#[derive(Debug)]
pub enum Extension {
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "ron")]
    Ron,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(feature = "json")]
                Extension::Json => "json",
                #[cfg(feature = "ron")]
                Extension::Ron => "ron",
                #[cfg(feature = "toml")]
                Extension::Toml => "toml",
                #[cfg(feature = "yaml")]
                Extension::Yaml => "yaml",
            }
        )
    }
}
