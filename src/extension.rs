use std::fmt::Display;

#[derive(Debug)]
pub enum Extension {
    #[cfg(feature = "ext_json")]
    Json,
    #[cfg(feature = "ext_ron")]
    Ron,
    #[cfg(feature = "ext_toml")]
    Toml,
    #[cfg(feature = "ext_yaml")]
    Yaml,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(feature = "ext_json")]
                Extension::Json => "json",
                #[cfg(feature = "ext_ron")]
                Extension::Ron => "ron",
                #[cfg(feature = "ext_toml")]
                Extension::Toml => "toml",
                #[cfg(feature = "ext_yaml")]
                Extension::Yaml => "yaml",
            }
        )
    }
}
