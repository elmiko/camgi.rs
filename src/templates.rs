use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    title: &'a str,
}

pub fn build_index_template(title: &str) -> IndexTemplate {
    IndexTemplate { title }
}
