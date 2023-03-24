pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use schema::posts::dsl::*;
use models::{Post};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "postgresql://postgres:NGmH3FBtu7BRgF0f2uAJ@containers-us-west-202.railway.app:6924/railway";
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn show_posts() -> Vec<models::Post> {

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    /*println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }*/
    return results;
}