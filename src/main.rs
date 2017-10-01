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
    let post_items = models::Post::get_posts();

    let post_handler = handlers::PostHandler {
        posts: post_items
    };

    let mut router = Router::new();

    router.get("/", post_handler, "index");
    router.get("/:query", handlers::query_handler, "query");

    Iron::new(router).http("localhost:3000").unwrap();
}