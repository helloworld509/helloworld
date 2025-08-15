use self::schema::posts::dsl::title;
use diesel::prelude::*;
use diesel_learning::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};
    let target = args().nth(1).expect("Expected a target to match against.");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting post");

    println!("deleted {} posts", num_deleted);
}
