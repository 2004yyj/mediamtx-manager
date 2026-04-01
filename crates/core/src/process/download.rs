use std::path::{Path, PathBuf};

use crate::error::CoreError;
use crate::models::process::BinaryInfo;

/// MediaMTX 바이너리 다운로더
pub struct BinaryDownloader {
    client: reqwest::Client,
    install_dir: PathBuf,
}

impl BinaryDownloader {
    pub fn new(install_dir: PathBuf) -> Self {
        Self {
            client: reqwest::Client::new(),
            install_dir,
        }
    }

    /// GitHub Releases에서 최신 버전 태그 조회
    pub async fn get_latest_version(&self) -> Result<String, CoreError> {
        let resp = self
            .client
            .get("https://api.github.com/repos/bluenviron/mediamtx/releases/latest")
            .header("User-Agent", "mediamtx-manager")
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Download(format!(
                "Failed to fetch latest version: {}",
                resp.status()
            )));
        }

        let body: serde_json::Value = resp.json().await?;
        body["tag_name"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| CoreError::Download("tag_name not found in response".into()))
    }

    /// 다운로드 URL 구성
    fn download_url(version: &str) -> Result<String, CoreError> {
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "windows",
            other => return Err(CoreError::Download(format!("Unsupported OS: {other}"))),
        };

        let arch = match std::env::consts::ARCH {
            "x86_64" | "x86" => "amd64",
            "aarch64" => "arm64v8",
            other => return Err(CoreError::Download(format!("Unsupported arch: {other}"))),
        };

        let ext = if os == "windows" { "zip" } else { "tar.gz" };

        // 버전 태그에서 'v' 프리픽스 제거
        let ver = version.strip_prefix('v').unwrap_or(version);

        Ok(format!(
            "https://github.com/bluenviron/mediamtx/releases/download/{version}/mediamtx_{ver}_{os}_{arch}.{ext}"
        ))
    }

    /// 바이너리 다운로드 및 설치
    pub async fn download(&self, version: Option<&str>) -> Result<BinaryInfo, CoreError> {
        let version = match version {
            Some(v) => v.to_string(),
            None => self.get_latest_version().await?,
        };

        let url = Self::download_url(&version)?;
        tracing::info!("Downloading MediaMTX from: {url}");

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "mediamtx-manager")
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CoreError::Download(format!(
                "Download failed: {} from {url}",
                resp.status()
            )));
        }

        let bytes = resp.bytes().await?;

        // 설치 디렉토리 생성
        tokio::fs::create_dir_all(&self.install_dir).await?;

        // 아카이브 압축 해제
        let install_dir = self.install_dir.clone();
        let archive_bytes = bytes.to_vec();

        let binary_name = if cfg!(windows) {
            "mediamtx.exe"
        } else {
            "mediamtx"
        };

        let dest = install_dir.clone();
        if url.ends_with(".tar.gz") {
            tokio::task::spawn_blocking(move || Self::extract_tar_gz(&archive_bytes, &dest))
                .await
                .map_err(|e| CoreError::Download(format!("Extract task failed: {e}")))??;
        } else {
            tokio::task::spawn_blocking(move || Self::extract_zip(&archive_bytes, &dest))
                .await
                .map_err(|e| CoreError::Download(format!("Extract task failed: {e}")))??;
        }

        let binary_path = install_dir.join(binary_name);
        if !binary_path.exists() {
            return Err(CoreError::Download(
                "Binary not found after extraction".into(),
            ));
        }

        // Unix: 실행 권한 설정
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(&binary_path, perms)?;
        }

        Ok(BinaryInfo {
            version,
            path: binary_path,
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        })
    }

    /// 설치된 바이너리 경로 반환 (존재하는 경우)
    pub fn installed_binary_path(&self) -> Option<PathBuf> {
        let name = if cfg!(windows) {
            "mediamtx.exe"
        } else {
            "mediamtx"
        };
        let path = self.install_dir.join(name);
        path.exists().then_some(path)
    }

    fn extract_tar_gz(data: &[u8], dest: &Path) -> Result<(), CoreError> {
        use std::io::Read;

        let decoder = flate2::read::GzDecoder::new(data);
        let mut archive = tar::Archive::new(decoder);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            let file_name = path
                .file_name()
                .ok_or_else(|| CoreError::Download("Invalid tar entry".into()))?;
            let dest_path = dest.join(file_name);

            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;
            std::fs::write(&dest_path, &buf)?;
        }

        Ok(())
    }

    fn extract_zip(data: &[u8], dest: &Path) -> Result<(), CoreError> {
        use std::io::Read;

        let cursor = std::io::Cursor::new(data);
        let mut archive = zip::ZipArchive::new(cursor)
            .map_err(|e| CoreError::Download(format!("Failed to open zip: {e}")))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| CoreError::Download(format!("Failed to read zip entry: {e}")))?;

            if file.is_dir() {
                continue;
            }

            let file_name = file
                .enclosed_name()
                .and_then(|p| p.file_name().map(|n| n.to_owned()))
                .ok_or_else(|| CoreError::Download("Invalid zip entry".into()))?;

            let dest_path = dest.join(file_name);
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            std::fs::write(&dest_path, &buf)?;
        }

        Ok(())
    }
}
