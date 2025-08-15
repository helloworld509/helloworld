use self::models::Post;
use diesel::prelude::*;
use diesel_learning::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;
    let connection = &mut establish_connection();
    let id = args()
        .nth(1)
        .expect("Please provide a post ID")
        .parse::<i32>()
        .expect("Post ID should be an integer");
    let post = posts
        .find(id)
        .first::<Post>(connection)
        .expect("Error loading post");
    println!(
        "Post ID: {}, Title: {}, Body: {}",
        post.id, post.title, post.body
    );
}
