use std::fmt::Display;

#[derive(Debug)]
pub enum Extension {
    #[cfg(feature = "ext_json")]
    Json,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(feature = "ext_json")]
                Extension::Json => "json",
            }
        )
    }
}
