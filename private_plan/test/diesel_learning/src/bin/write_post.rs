use diesel::prelude::*;
use diesel_learning::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();
    let mut title = String::new();
    let mut body = String::new();

    println!("Enter post title:");
    stdin().read_line(&mut title).expect("Failed to read line");
    let title = title.trim();
    println!("Enter post body:");
    stdin().read_line(&mut body).expect("Failed to read line");
    let body = body.trim();

    let post = create_post(connection, title, body, false);
    println!("Created post: {} with ID {}", post.title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";
#[cfg(windows)]
const EOF: &str = "CTRL+Z";
