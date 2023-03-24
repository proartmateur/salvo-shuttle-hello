mod user;
mod db;

use crate::db::show_posts;
use crate::user::controllers::get::get_user;

//# region DB Connection
use salvo::prelude::*;
use serde::Serialize;
//# endregion

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[handler]
async fn hello_world(res: &mut Response) {
    let posts = show_posts();

    /*for post in posts {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }*/
    res.render(Json(posts));
}

#[shuttle_runtime::main]
async fn salvo() -> shuttle_salvo::ShuttleSalvo {

    //let router = Router::with_path("hello").get(hello_world);
    let router = Router::new()
        .push(Router::with_path("hello").get(hello_world))
        .get(get_user);

    Ok(router.into())
}