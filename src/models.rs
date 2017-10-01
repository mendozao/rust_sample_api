use database;

use diesel::prelude::*;

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


impl Post {
    pub fn get_posts() -> Vec<Post> {
        use schema::posts::dsl::*;

        let connection = database::establish_connection();

        let results = posts.filter(published.eq(true))
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts");
        results
    }
}
