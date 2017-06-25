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

    html! {
        h1 { "Post 一覧表示" }
        ol {
            @for post in results {
                li {
                    (post.title) " " (post.body)
                }
            }
        }
    }
}
