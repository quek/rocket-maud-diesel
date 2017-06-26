use rocket::response::Redirect;
use diesel::prelude::*;

use maud::Markup;
use models::Post;
use db;
use layouts;

#[get("/")]
pub fn index(connection: db::Conn) -> Markup {
    use schema::posts::dsl::*;

    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&*connection)
        .expect("Error loading posts");

    layouts::default("ポスト一覧",
                     html! {
                         h1 { "Post 一覧表示" }
                         p a href="/posts/new" "新規登録"
                         ol {
                             @for post in results {
                                 li {
                                     (post.title) " " (post.body)
                                 }
                             }
                         }
                     })
}

#[get("/new")]
pub fn new() -> Markup {
    let x = html! {
        form action="/posts" method="post" {
            h1 "新規登録"
            p {
                label "タイトル"
                input type="text" name="title" /
            }
            p {
                label "本文"
                textarea name="body" {}
            }
            input type="submit" value="登録" /
        }
    };
    layouts::default("ポスト 新規登録", x)
}

#[post("/")]
pub fn create(connection: db::Conn) -> Redirect {
    Redirect::to("/posts")
}
