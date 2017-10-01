use iron::status;
use iron::IronResult;
use iron::Response;
use iron::Request;

use models;

use router::Router;

use rustc_serialize;


pub fn posts_handler(_: &mut Request) -> IronResult<Response> {
    let post_items = models::Post::get_posts();
    let encoded = rustc_serialize::json::encode(&post_items).unwrap();
    Ok(Response::with((status::Ok, encoded)))
}

pub fn post_handler(req: &mut Request) -> IronResult<Response> {
    let ref post_id_query = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("/");
    let post_id = post_id_query.parse::<i32>().unwrap();
    let post_item = models::Post::get_post(post_id);
    let encoded = rustc_serialize::json::encode(&post_item).unwrap();
    Ok(Response::with((status::Ok, encoded)))
}