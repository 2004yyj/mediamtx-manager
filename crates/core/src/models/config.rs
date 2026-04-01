use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MediaMTX 글로벌 설정.
/// 주요 필드만 타이핑하고 나머지는 extra로 유연하게 처리.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destinations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_buffer_count: Option<i32>,

    // API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_address: Option<String>,

    // RTSP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp_address: Option<String>,

    // RTMP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_address: Option<String>,

    // HLS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_address: Option<String>,

    // WebRTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webrtc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webrtc_address: Option<String>,

    // SRT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt_address: Option<String>,

    // Metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_address: Option<String>,

    // Record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_format: Option<String>,

    /// 타이핑되지 않은 나머지 필드들
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// MediaMTX 경로(Path) 설정
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PathConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_on_demand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_on_demand_start_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_on_demand_close_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_format: Option<String>,

    // Hooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_on_init: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_on_ready: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_on_read: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_on_unread: Option<String>,

    /// 타이핑되지 않은 나머지 필드들
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 경로 설정 목록 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathConfigList {
    pub page_count: i32,
    pub items: Vec<PathConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_config_deserialize_partial() {
        let json = r#"{"logLevel":"info","api":true,"apiAddress":":9997","unknownField":42}"#;
        let config: GlobalConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.log_level.as_deref(), Some("info"));
        assert_eq!(config.api, Some(true));
        assert_eq!(config.api_address.as_deref(), Some(":9997"));
        assert_eq!(config.extra.get("unknownField").unwrap(), &serde_json::json!(42));
    }

    #[test]
    fn path_config_serialize_skip_none() {
        let config = PathConfig {
            source: Some("rtsp://example.com/stream".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("source"));
        assert!(!json.contains("name"));
    }
}
