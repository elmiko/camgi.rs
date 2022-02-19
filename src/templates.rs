use crate::mustgather::MustGather;

use anyhow::Result;
use html_builder::*;
use std::fmt::Write;

// Create the <head> node of the html document
fn add_head(html: &mut Node, title: String) -> Result<()> {
    let mut head = html.head();
    writeln!(head.title(), "{}", &title)?;
    head.meta().attr("charset='utf-8'");
    head.link().attr(
        "href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/css/bootstrap.min.css\"",
    )
    .attr("rel=\"stylesheel\"")
    .attr("integrity=\"sha384-eOJMYsd53ii+scO/bJGFsiCZc+5NDVN2yr8+0RDqr0Ql0h+rP48ckxlpbzKgwra6\"")
    .attr("crossorigin=\"anonymous\"");
    head.style()
        .write_str(include_str!("../templates/styles.css"))?;
    Ok(())
}

// Demo how to create html fragment
fn add_table(node: &mut Node, headers: Vec<&str>, _data: Vec<Vec<&str>>) -> Result<()> {
    let mut table = node
        .table()
        .attr("class=\"table table-sm table-striped font-monospace\"");
    let mut thead = table.thead();
    let mut tr = thead.tr();
    for header in headers {
        let mut th = tr.th().attr("scope=\"col\"");
        writeln!(th, "{}", header)?;
    }
    Ok(())
}

// Demo how to compose html nodes
fn add_summary(html: &mut Node) -> Result<()> {
    let mut data = html.data().attr("id=\"summary-data\"");
    add_table(
        &mut data,
        vec!["Resources", "Allocatable", "Capacity"],
        vec![],
    )?;
    Ok(())
}

/// Create the html buffer
pub fn build_index(mustgather: MustGather) -> Result<Buffer> {
    let mut buf = Buffer::new();
    let mut html = buf.html().attr("lang='en'");

    add_head(&mut html, mustgather.title())?;
    add_summary(&mut html)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_index() {
        let mg = MustGather {
            path: PathBuf::from("/bar"),
        };
        let index = build_index(mg).unwrap();
        assert_eq!(index.finish(), include_str!("../templates/demo.html"))
    }
}
