use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    basename: &'a str,
}

pub fn build_index_template(basename: &str) -> IndexTemplate {
    IndexTemplate {
        basename: basename,
    }
}
