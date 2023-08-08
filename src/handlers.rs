use axum::debug_handler;
use axum::response::{IntoResponse, Response};
use http::header::CONTENT_TYPE;
use http::{HeaderValue, StatusCode};
use mime::APPLICATION_JSON;
use serde::Serialize;
use time::OffsetDateTime;
use tracing::error;

use crate::app::context::RefContext;

pub async fn epoch() -> Response {
    let ts = OffsetDateTime::now_utc().unix_timestamp();
    ts.to_string().into_response()
}

#[debug_handler]
pub async fn root(context: RefContext) -> Response {
    match root_impl(context).await {
        Ok(value) => value.into_response(),
        Err(err) => {
            error!("error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RootResponse {
    hostname: String,
    server_name: Option<String>,
}

async fn root_impl(context: RefContext) -> crate::Result<Response> {
    let hostname = gethostname::gethostname().to_string_lossy().into_owned();
    let server_name = context.config.server_name.clone();
    let response = RootResponse {
        hostname,
        server_name,
    };
    let mut response = serde_json::to_string_pretty(&response)?.into_response();
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static(APPLICATION_JSON.essence_str()),
    );
    Ok(response)
}
