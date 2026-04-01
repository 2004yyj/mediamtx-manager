use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use mediamtx_manager_core::CoreError;

pub struct AppError(pub CoreError);

impl From<CoreError> for AppError {
    fn from(err: CoreError) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            CoreError::Api { status, .. } => {
                StatusCode::from_u16(*status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            CoreError::BinaryNotFound(_) => StatusCode::NOT_FOUND,
            CoreError::ConfigFile(_) => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({ "error": self.0.to_string() });
        (status, Json(body)).into_response()
    }
}
