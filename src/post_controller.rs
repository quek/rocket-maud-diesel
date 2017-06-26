use rocket::response::Redirect;
use rocket::request::Form;
use diesel;
use diesel::prelude::*;

use maud::Markup;
use models::{Post, NewPost};
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
            p {
                label {
                    input type="radio" name="published" value="true"
                    "公開"
                }
                label {
                    input type="radio" name="published" value="false"
                    "非公開"
                }
            }
            input type="submit" value="登録" /
        }
    };
    layouts::default("ポスト 新規登録", x)
}

#[derive(FromForm)]
pub struct PostForm {
    title: String,
    body: String,
    published: bool,
}

#[post("/", data="<form>")]
pub fn create(form: Form<PostForm>, connection: db::Conn) -> Redirect {
    use schema::posts;

    let new_post = NewPost {
        title: &form.get().title,
        body: &form.get().body,
        published: form.get().published,
    };
    diesel::insert(&new_post)
        .into(posts::table)
        .execute(&*connection)
        .expect("Error saving new post");

    Redirect::to("/posts")
}
