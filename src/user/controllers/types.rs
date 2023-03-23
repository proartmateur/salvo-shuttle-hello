use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub name: String,
    pub id: i64
}

#[derive(Serialize, Debug)]
pub struct UserSerializationErrorResponse {
    pub message: String,
}