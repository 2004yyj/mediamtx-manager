use serde::{Deserialize, Serialize};

/// 활성 경로의 소스 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceInfo {
    #[serde(rename = "type")]
    pub source_type: Option<String>,
    pub id: Option<String>,
}

/// 활성 경로의 리더 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReaderInfo {
    #[serde(rename = "type")]
    pub reader_type: Option<String>,
    pub id: Option<String>,
}

/// 활성 경로 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    pub name: String,
    pub source: Option<SourceInfo>,
    pub readers: Option<Vec<ReaderInfo>>,
    pub ready: Option<bool>,
    pub bytes_received: Option<u64>,
    pub bytes_sent: Option<u64>,
}

/// 활성 경로 목록 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathList {
    pub page_count: i32,
    pub items: Vec<PathItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_item_deserialize() {
        let json = r#"{
            "name": "stream1",
            "ready": true,
            "source": {"type": "rtspSession", "id": "abc123"},
            "readers": [{"type": "rtspSession", "id": "def456"}],
            "bytesReceived": 1024,
            "bytesSent": 512
        }"#;
        let item: PathItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.name, "stream1");
        assert_eq!(item.ready, Some(true));
        assert_eq!(item.bytes_received, Some(1024));
        assert!(item.source.is_some());
        assert_eq!(item.readers.as_ref().unwrap().len(), 1);
    }
}
