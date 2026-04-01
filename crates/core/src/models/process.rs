use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// MediaMTX 프로세스 상태
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ProcessStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error { message: String },
}

/// 설치된 MediaMTX 바이너리 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryInfo {
    pub version: String,
    pub path: PathBuf,
    pub os: String,
    pub arch: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_status_serialize() {
        let status = ProcessStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#"{"status":"running"}"#);

        let error = ProcessStatus::Error {
            message: "failed to start".into(),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("failed to start"));
    }

    #[test]
    fn process_status_deserialize() {
        let json = r#"{"status":"running"}"#;
        let status: ProcessStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, ProcessStatus::Running);
    }
}
