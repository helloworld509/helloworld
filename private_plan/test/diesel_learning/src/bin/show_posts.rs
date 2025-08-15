use self::models::*;
use diesel::prelude::*;
use diesel_learning::*;

fn main() {
    use schema::posts;
    let connection = &mut establish_connection();
    let results = posts::table
        .filter(posts::published.eq(true))
        .load::<Post>(connection)
        .expect("Error loading posts");
    println!("Loaded {} posts", results.len());
    for post in results {
        println!("{}: {}", post.title, post.body);
    }
}
