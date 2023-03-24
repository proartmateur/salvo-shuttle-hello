mod user;
mod db;

use db::show_posts;

use diesel::{PgConnection};
//use crate::db::{};
use crate::user::controllers::get::get_user;

//# region DB Connection
use salvo::prelude::*;
//use serde::Serialize;


//use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use once_cell::sync::OnceCell;
//use self::schema::posts::dsl::*;

const DB_URL: &str = "postgresql://postgres:NGmH3FBtu7BRgF0f2uAJ@containers-us-west-202.railway.app:6924/railway";

type PgPool = Pool<ConnectionManager<PgConnection>>;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

fn connect() -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
    return DB_POOL.get().unwrap().get()
}

fn build_pool(database_url: &str, size: u32) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    diesel::r2d2::Pool::builder()
        .max_size(size)
        .min_idle(Some(size))
        .test_on_check_out(false)
        .idle_timeout(None)
        .max_lifetime(None)
        .build(manager)
}


//# endregion



#[handler]
async fn hello_world(res: &mut Response) {
    let conn = connect();
    match conn {
        Ok(mut _v) => res.render(Json(show_posts(&mut _v))),
        Err(_e) => res.render(Text::Plain("err"))
    }
    //let posts = show_posts();

    /*
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");
    */
    //res.render(Json(posts));
}

#[shuttle_runtime::main]
async fn salvo() -> shuttle_salvo::ShuttleSalvo {
    DB_POOL
        .set(build_pool(&DB_URL, 10).expect(&format!("Error connecting to {}", &DB_URL)))
        .ok();

    //let hi_w = hello_closure(connection);
    //let router = Router::with_path("hello").get(hello_world);
    let router = Router::new()
        .push(Router::with_path("hello").get(hello_world))
        .get(get_user);

    Ok(router.into())
}