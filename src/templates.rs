use crate::mustgather::MustGather;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    title: String,
}

pub fn build_index_template(mustgather: MustGather) -> IndexTemplate {
    IndexTemplate {
        title: mustgather.title(),
    }
}
