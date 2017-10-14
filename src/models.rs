use database;

use diesel::prelude::*;


#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Post {
    #[serde(skip_serializing)]
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn get_post(post_id: i32) -> Post {
        use schema::posts::dsl::{posts};
        
        let connection = database::establish_connection();

        let result = posts.find(post_id).first::<Post>(&connection).expect("Error loading post");
        result
    }

    pub fn get_posts() -> Vec<Post> {
        use schema::posts::dsl::{posts, published};
        
        let connection = database::establish_connection();

        let results = posts.filter(published.eq(true))
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts");
        results
    }
}
