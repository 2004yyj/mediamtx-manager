use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;

/// MediaMTX의 HLS/WebRTC 포트를 프록시하여 cross-origin 문제 해결
pub fn routes() -> Router {
    Router::new()
        .route("/whep/{*path}", any(proxy_webrtc))
        .route("/hls/{*path}", any(proxy_hls))
}

async fn proxy_webrtc(req: Request) -> impl IntoResponse {
    proxy_to("http://127.0.0.1:8889", req).await
}

async fn proxy_hls(req: Request) -> impl IntoResponse {
    proxy_to("http://127.0.0.1:8888", req).await
}

async fn proxy_to(base: &str, req: Request) -> Response {
    let client = reqwest::Client::new();

    let path = req.uri().path();
    // /whep/stream/whep → /stream/whep, /hls/stream/index.m3u8 → /stream/index.m3u8
    let target_path = if let Some(rest) = path.strip_prefix("/whep") {
        rest
    } else if let Some(rest) = path.strip_prefix("/hls") {
        rest
    } else {
        path
    };

    let url = format!("{}{}", base, target_path);
    let method = req.method().clone();
    let headers = req.headers().clone();
    let body = axum::body::to_bytes(req.into_body(), 64 * 1024)
        .await
        .unwrap_or_default();

    let mut upstream_req = client.request(method, &url);

    // Content-Type 헤더 전달
    if let Some(ct) = headers.get("content-type") {
        upstream_req = upstream_req.header("content-type", ct);
    }

    let resp = match upstream_req.body(body).send().await {
        Ok(r) => r,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from(format!("proxy error: {e}")))
                .unwrap();
        }
    };

    let status = resp.status();
    let resp_headers = resp.headers().clone();
    let resp_body = resp.bytes().await.unwrap_or_default();

    let mut response = Response::builder().status(status.as_u16());

    // 주요 헤더 전달 (location은 별도 처리)
    let forward_headers = ["content-type", "etag", "accept-patch", "link", "id"];
    for key in forward_headers {
        if let Some(val) = resp_headers.get(key) {
            response = response.header(key, val);
        }
    }

    // Location 헤더의 경로를 프록시 경로로 변환
    if let Some(loc) = resp_headers.get("location") {
        if let Ok(loc_str) = loc.to_str() {
            let proxied_loc = format!("/whep{}", loc_str);
            response = response.header("location", proxied_loc);
        }
    }

    response.body(Body::from(resp_body)).unwrap()
}
