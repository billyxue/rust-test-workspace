use serde::Serialize;
use crate::api::response::error::Status;
use utoipa::ToSchema;

#[derive(Serialize,ToSchema)]
pub struct LoginResponse {
    //pub status: String,
    pub status: Status,
    pub token: String,
}
