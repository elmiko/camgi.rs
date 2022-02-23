use crate::mustgather::MustGather;

use anyhow::Result;
use html_builder::*;
use std::fmt::Write;

pub fn build_html(mustgather: MustGather) -> Result<String> {
    let mut buf = Buffer::new();

    Ok(buf.finish())
}
