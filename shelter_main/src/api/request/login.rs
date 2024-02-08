use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Deserialize,ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

