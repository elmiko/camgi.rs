use crate::mustgather::MustGather;

use anyhow::Result;
use html_builder::*;
use std::fmt::Write;

pub fn build_html(mustgather: MustGather) -> Result<String> {
    let mut buf = Buffer::new();
    let mut html = buf.html().attr("lang='en'");

    add_head(&mut html, &mustgather)?;
    add_body(&mut html, &mustgather)?;

    Ok(buf.finish())
}

fn add_head(html: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut head = html.head();
    writeln!(head.title(), "{}", mustgather.title())?;
    head.meta().attr("charset='utf-8'");
    head.link()
        .attr("href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/css/bootstrap.min.css\"")
        .attr("rel=\"stylesheet\"")
        .attr("integrity=\"sha384-eOJMYsd53ii+scO/bJGFsiCZc+5NDVN2yr8+0RDqr0Ql0h+rP48ckxlpbzKgwra6\"")
        .attr("crossorigin=\"anonymous\"");
    head.style()
        .write_str(include_str!("../templates/style.css"))?;
    Ok(())
}

fn add_body(html: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut body = html.body();

    Ok(())
}
