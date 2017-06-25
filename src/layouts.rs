use maud::{DOCTYPE, Markup};

pub fn default(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title (title)
            }
            body (content)
        }
    }
}
