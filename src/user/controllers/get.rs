
use super::types::{
    User
};

use super::helpers::{
    process_deserialize_error,
    process_response_json
};

use salvo::{prelude::*};

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    //let id = req.query::<User>("id");

    //let user: User = req.parse_queries();
    let version: Result<User, salvo::http::ParseError> = req.parse_queries();
    match version {
        Ok(_v) => process_response_json(res, &_v.id),
        Err(_e) => process_deserialize_error(res, _e)
    }
}