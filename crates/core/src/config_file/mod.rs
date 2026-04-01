pub mod hot_reload;

use std::path::{Path, PathBuf};

use crate::error::CoreError;

/// MediaMTX 설정 파일(YAML) 관리자
pub struct ConfigFileManager {
    config_path: PathBuf,
}

impl ConfigFileManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    pub fn path(&self) -> &Path {
        &self.config_path
    }

    pub fn exists(&self) -> bool {
        self.config_path.exists()
    }

    /// 설정 파일을 문자열로 읽기
    pub async fn read_as_string(&self) -> Result<String, CoreError> {
        tokio::fs::read_to_string(&self.config_path)
            .await
            .map_err(|e| CoreError::ConfigFile(format!("Failed to read config: {e}")))
    }

    /// 설정 파일을 파싱된 YAML Value로 읽기
    pub async fn read(&self) -> Result<serde_yaml::Value, CoreError> {
        let content = self.read_as_string().await?;
        Ok(serde_yaml::from_str(&content)?)
    }

    /// 문자열 내용을 설정 파일로 쓰기
    pub async fn write_string(&self, content: &str) -> Result<(), CoreError> {
        // 부모 디렉토리 생성
        if let Some(parent) = self.config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&self.config_path, content)
            .await
            .map_err(|e| CoreError::ConfigFile(format!("Failed to write config: {e}")))
    }

    /// YAML Value를 설정 파일로 쓰기
    pub async fn write(&self, value: &serde_yaml::Value) -> Result<(), CoreError> {
        let content =
            serde_yaml::to_string(value).map_err(|e| CoreError::ConfigFile(e.to_string()))?;
        self.write_string(&content).await
    }
}
