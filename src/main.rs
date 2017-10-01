extern crate dotenv;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use iron::Iron;

use router::Router;

pub mod database;
pub mod handlers;
pub mod models;
pub mod schema;


fn main() {
    let mut router = Router::new();

    router.get("/posts", handlers::posts_handler, "posts_list");
    router.get("/posts/:id", handlers::post_handler, "posts_detail");

    Iron::new(router).http("localhost:3000").unwrap();
}