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
                                     a href={ "/posts/" (post.id) } "編集"
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

#[get("/<id>")]
pub fn edit(id: i32, connection: db::Conn) -> Markup {
    use schema::posts::dsl::posts;

    let post = posts.find(id).get_result::<Post>(&*connection).unwrap();

    let x = html! {
        h1 "編集"
        form action={"/posts/" (id)} method="post" {
            p {
                label "タイトル"
                input type="text" name="title" value=(post.title) /
            }
            p {
                label "本文"
                textarea name="body" (post.body)
            }
            p {
                label {
                    input type="radio" name="published" value="true" checked?[post.published] /
                    "公開"
                }
                label {
                    input type="radio" name="published" value="false" checked?[!post.published] /
                    "非公開"
                }
            }
            input type="submit" value="更新" /
        }
    };
    layouts::default("ポスト 編集", x)
}

#[post("/<id>", data="<form>")]
pub fn update(id: i32, form: Form<PostForm>, connection: db::Conn) -> Redirect {
    use schema::posts::dsl::{posts, title, body, published};

    let form = form.get();
    let post = posts.find(id);
    diesel::update(post)
        .set((title.eq(&form.title), body.eq(&form.body), published.eq(form.published)))
        .get_result::<Post>(&*connection)
        .expect("更新失敗");

    Redirect::to("/posts")
}
