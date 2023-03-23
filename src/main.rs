mod user;
use salvo::{prelude::*};
use crate::user::controllers::get::get_user;

#[handler]
async fn hello_world(res: &mut Response) {
    res.render(Text::Plain("Hello, world!"));
}

#[shuttle_runtime::main]
async fn salvo() -> shuttle_salvo::ShuttleSalvo {
    //let router = Router::with_path("hello").get(hello_world);
    let router = Router::new()
        .push(Router::with_path("hello").get(hello_world))
        .get(get_user);

    Ok(router.into())
}