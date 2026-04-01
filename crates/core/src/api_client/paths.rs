use crate::error::CoreError;
use crate::models::config::{PathConfig, PathConfigList};
use crate::models::path::{PathItem, PathList};

use super::MediaMtxClient;

impl MediaMtxClient {
    // ── 경로 설정 CRUD ──

    /// 설정된 모든 경로 목록 조회
    pub async fn list_path_configs(&self) -> Result<PathConfigList, CoreError> {
        let resp = self
            .client()
            .get(self.url("/v3/config/paths/list"))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(resp.json().await?)
    }

    /// 특정 경로 설정 조회
    pub async fn get_path_config(&self, name: &str) -> Result<PathConfig, CoreError> {
        let resp = self
            .client()
            .get(self.url(&format!("/v3/config/paths/get/{name}")))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(resp.json().await?)
    }

    /// 새 경로 추가
    pub async fn add_path_config(
        &self,
        name: &str,
        config: &PathConfig,
    ) -> Result<(), CoreError> {
        let resp = self
            .client()
            .post(self.url(&format!("/v3/config/paths/add/{name}")))
            .json(config)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    /// 특정 경로 설정 부분 수정
    pub async fn update_path_config(
        &self,
        name: &str,
        config: &PathConfig,
    ) -> Result<(), CoreError> {
        let resp = self
            .client()
            .patch(self.url(&format!("/v3/config/paths/patch/{name}")))
            .json(config)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    /// 특정 경로 삭제
    pub async fn delete_path_config(&self, name: &str) -> Result<(), CoreError> {
        let resp = self
            .client()
            .post(self.url(&format!("/v3/config/paths/delete/{name}")))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    // ── 활성 경로 조회 ──

    /// 활성 경로 목록 조회
    pub async fn list_paths(&self) -> Result<PathList, CoreError> {
        let resp = self
            .client()
            .get(self.url("/v3/paths/list"))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(resp.json().await?)
    }

    /// 특정 활성 경로 조회
    pub async fn get_path(&self, name: &str) -> Result<PathItem, CoreError> {
        let resp = self
            .client()
            .get(self.url(&format!("/v3/paths/get/{name}")))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(resp.json().await?)
    }
}
