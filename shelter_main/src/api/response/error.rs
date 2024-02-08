use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/*
#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
*/

#[derive(Serialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

//pub struct AppError(anyhow::Error);
pub struct AppError(pub StatusCode, pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            /*
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                status: "error".to_string(),
                message: self.0.to_string(),
            }),
            */
            self.0,
            Json(ErrorResponse {
                status: Status::Error,
                message: self.1.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        //Self(err.into())
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}
