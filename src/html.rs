use crate::prelude::*;
use crate::resources::Resource;
use html_builder::*;
use std::fmt::Write;

pub struct Html {
    buffer: Buffer,
}

impl Html {
    pub fn from(mustgather: MustGather) -> Result<Html> {
        let mut buffer = Buffer::new();
        let mut html = buffer.html().attr("lang='en'");

        add_head(&mut html, &mustgather)?;
        add_body(&mut html, &mustgather)?;

        Ok(Html { buffer })
    }
}

impl Html {
    pub fn render(self) -> String {
        self.buffer.finish()
    }
}

fn add_body(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut body = parent.body();

    // main div
    let mut app = body
        .div()
        .attr("id=\"app\"")
        .attr("class=\"container-fluid\"");
    let mut row = app.div().attr("class=\"row mt-2\"");

    // nav list
    let mut nav = row.div().attr("class=\"col-2\"").attr("id=\"nav-col\"");
    let mut navlist = nav.div().attr("class=\"list-group\"");

    navlist
        .a()
        .attr("href=\"#\"")
        .attr("v-on:click=\"changeContent('summary')\"")
        .attr("class=\"list-group-item list-group-item-action\"")
        .write_str("Summary")?;

    add_navlist_entry(&mut navlist, "Nodes", &mustgather.nodes)?;

    // github link should go last
    navlist
        .a()
        .attr("href=\"https://github.com/elmiko/camgi.rs\"")
        .attr("class=\"list-group-item list-group-item-action text-center\"")
        .attr("target=\"_blank\"")
        .img()
        .attr("src=\"https://github.com/favicon.ico\"")
        .attr("alt=\"GitHub logo\"")
        .attr("title=\"Found a bug or issue? Visit this project's git repo.\"");

    // content
    let mut content = row.div().attr("class=\"col-10\"");
    content
        .div()
        .attr("id=\"main-content\"")
        .attr("class=\"overflow-auto\"")
        .span()
        .attr("v-html=\"content\"");

    // add data sections
    add_summary_data(&mut body, &mustgather)?;
    add_resource_data(&mut body, "Nodes", &mustgather.nodes)?;

    // scripts
    body.script()
        .attr("src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/js/bootstrap.bundle.min.js\"")
        .attr("integrity=\"sha384-JEW9xMcG8R+pH31jmWH6WWP0WintQrMb4s7ZOdauHnUtxwoG2vI5DkLtS3qm9Ekf\"")
        .attr("crossorigin=\"anonymous\"");
    body.script()
        .attr("src=\"https://cdn.jsdelivr.net/npm/vue@2/dist/vue.js\"");
    body.script()
        .write_str(include_str!("files/index_script.js"))?;

    Ok(())
}

fn add_head(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut head = parent.head();
    head.title().write_str(mustgather.title.as_str())?;
    head.meta().attr("charset='utf-8'");
    head.link()
        .attr("href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/css/bootstrap.min.css\"")
        .attr("rel=\"stylesheet\"")
        .attr("integrity=\"sha384-eOJMYsd53ii+scO/bJGFsiCZc+5NDVN2yr8+0RDqr0Ql0h+rP48ckxlpbzKgwra6\"")
        .attr("crossorigin=\"anonymous\"");
    head.style()
        .write_str(include_str!("files/index_style.css"))?;
    Ok(())
}

fn add_navlist_entry(parent: &mut Node, title: &str, resources: &Vec<impl Resource>) -> Result<()> {
    let mut a = parent
        .a()
        .attr("href=\"#\"")
        .attr(format!("v-on:click=\"changeContent('{}')\"", title.to_lowercase()).as_str())
        .attr("class=\"list-group-item list-group-item-action\"");
    a.write_str(title)?;

    let errors = resources.iter().filter(|r| r.is_error()).count();
    if errors > 0 {
        a.span()
            .attr("class=\"badge bg-danger float-right\"")
            .write_str(format!("{}", errors).as_str())?;
    }
    Ok(())
}

fn add_resource_data(parent: &mut Node, kind: &str, resources: &Vec<impl Resource>) -> Result<()> {
    let mut data = parent
        .data()
        .attr(format!("id=\"{}-data\"", kind.to_lowercase()).as_str());
    data.h1().write_str(kind)?;
    let mut div = data
        .div()
        .attr("class=\"accordion\"")
        .attr(format!("id=\"{}-accordion\"", kind.to_lowercase()).as_str());
    for res in resources {
        let mut itemdiv = div.div().attr("class=\"accordion-item\"");
        let buttonclass = if res.is_error() {
            " bg-danger text-white"
        } else {
            ""
        };
        itemdiv
            .h2()
            .attr("class=\"accordion-header\"")
            .attr(format!("id=\"heading-{}\"", &res.safename()).as_str())
            .button()
            .attr(format!("class=\"accordion-button collapsed p-2{}\"", buttonclass).as_str())
            .attr("type=\"button\"")
            .attr("data-bs-toggle=\"collapse\"")
            .attr(format!("data-bs-target=\"#collapse-{}\"", &res.safename()).as_str())
            .attr("aria-exapnded=\"false\"")
            .attr(format!("aria-controls=\"collapse-{}\"", &res.safename()).as_str())
            .write_str(&res.name())?;
        itemdiv
            .div()
            .attr(format!("id=\"collapse-{}\"", &res.safename()).as_str())
            .attr("class=\"accordion-collapse collapse\"")
            .attr(format!("aria-labelledby=\"heading-{}\"", &res.safename()).as_str())
            .attr(format!("data-bs-parents=\"{}-accordion\"", kind.to_lowercase()).as_str())
            .div()
            .attr("class=\"accordion-body fs-6\"")
            .pre()
            .write_str(&res.raw())?;
    }

    Ok(())
}

fn add_summary_data(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut data = parent.data().attr("id=\"summary-data\"");
    data.h1().write_str("Summary")?;
    data.hr();
    let mut dl = data.dl();

    dl.dt()
        .attr("class=\"text-light bg-secondary ps-1 mb-1\"")
        .write_str("Cluster")?;
    let mut dd = dl.dd();
    add_table(
        &mut dd,
        Vec::new(),
        vec!["OpenShift Version", mustgather.version.as_str()],
    )?;

    dl.dt()
        .attr("class=\"text-light bg-secondary ps-1 mb-1\"")
        .write_str(format!("{} Nodes", mustgather.nodes.len()).as_str())?;
    let mut dd = dl.dd();
    let notready: Vec<String> = mustgather
        .nodes
        .iter()
        .filter(|n| n.is_error())
        .map(|n| n.name())
        .cloned()
        .collect();
    if notready.len() > 0 {
        add_table(
            &mut dd,
            vec!["Nodes not ready"],
            notready.iter().map(|n| n.as_str()).collect(),
        )?;
    } else {
        dd.write_str("All nodes ready")?;
    }

    Ok(())
}

fn add_table(parent: &mut Node, head: Vec<&str>, body: Vec<&str>) -> Result<()> {
    let mut table = parent
        .table()
        .attr("class=\"table table-sm table-striped font-monospace\"");

    if !head.is_empty() {
        let mut thead = table.thead();
        let mut tr = thead.tr();
        for (i, item) in head.iter().enumerate() {
            let t = if i == 0 { tr.th() } else { tr.td() };
            t.attr("scope=\"col\"").write_str(item)?;
        }
    }

    let mut tbody = table.tbody();
    let mut tr = tbody.tr();
    for (i, item) in body.iter().enumerate() {
        let t = if i == 0 { tr.th() } else { tr.td() };
        t.attr("scope=\"col\"").write_str(item)?;
    }

    Ok(())
}
