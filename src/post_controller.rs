use diesel::prelude::*;

use maud::Markup;
use models::Post;
use db;

#[get("/posts")]
pub fn index(connection: db::Conn) -> Markup {
    use schema::posts::dsl::*;

    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&*connection)
        .expect("Error loading posts");
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    html! {
        h1 { "Posts" }
        p "一覧表示"
    }
}
