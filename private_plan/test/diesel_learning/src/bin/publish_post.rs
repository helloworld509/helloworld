use self::models::*;
use diesel::prelude::*;
use diesel_learning::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};
    let id = args()
        .nth(1)
        .expect("Please provide a post ID")
        .parse::<i32>()
        .expect("Post ID should be an integer");
    let connection = &mut establish_connection();
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .expect("Error publishing post");
    println!("Published post: {} with ID {}", post.title, post.id);
}
