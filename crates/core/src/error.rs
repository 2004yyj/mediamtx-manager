use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("MediaMTX API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Process error: {0}")]
    Process(String),

    #[error("Binary not found: {0}")]
    BinaryNotFound(String),

    #[error("Download error: {0}")]
    Download(String),

    #[error("Config file error: {0}")]
    ConfigFile(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

impl Serialize for CoreError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
