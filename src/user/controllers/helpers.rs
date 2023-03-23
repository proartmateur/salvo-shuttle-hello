
use super::types::{
    UserResponse, 
    UserSerializationErrorResponse
};

use salvo::{prelude::*, http::ParseError};
use std::*;

pub fn process_deserialize_error( res: &mut Response, _err: ParseError) {
    res.set_status_code(StatusCode::BAD_REQUEST);
    res.render(Json(UserSerializationErrorResponse{message: "Problema al deserializar".to_string()}));
}

pub fn process_response_json(res: &mut Response, id: &i64) {
    let user = UserResponse{
        name: "jobs get".to_string(),
        id: id.clone()
    };
    res.render(Json(user));
}
