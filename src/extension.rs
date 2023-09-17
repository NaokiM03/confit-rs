use std::fmt::Display;

#[derive(Debug)]
pub enum Extension {
    #[cfg(feature = "ext_json")]
    Json,
}

impl Default for Extension {
    fn default() -> Self {
        Extension::Json
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Extension::Json => "json",
            }
        )
    }
}
