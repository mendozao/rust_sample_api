use iron::Handler;
use iron::status;
use iron::IronResult;
use iron::Response;
use iron::Request;

use models;

use router::Router;

use rustc_serialize;

pub struct PostHandler {
    pub posts: Vec<models::Post>
}

impl Handler for PostHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let encoded = rustc_serialize::json::encode(&self.posts).unwrap();
        Ok(Response::with((status::Ok, encoded)))
    }
}

pub fn query_handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}