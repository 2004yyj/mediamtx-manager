use crate::error::CoreError;
use crate::models::config::GlobalConfig;

use super::MediaMtxClient;

impl MediaMtxClient {
    /// 글로벌 설정 조회
    pub async fn get_global_config(&self) -> Result<GlobalConfig, CoreError> {
        let resp = self
            .client()
            .get(self.url("/v3/config/global/get"))
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

    /// 글로벌 설정 부분 수정
    pub async fn patch_global_config(&self, config: &GlobalConfig) -> Result<(), CoreError> {
        let resp = self
            .client()
            .patch(self.url("/v3/config/global/patch"))
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
}
