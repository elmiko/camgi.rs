use crate::mustgather::MustGather;
use anyhow::Result;
use html_builder::*;
use std::fmt::Write;

pub struct Html {
    buffer: Buffer,
}

impl Html {
    pub fn from(mustgather: MustGather) -> Result<Html> {
        let mut html = Html {
            buffer: Buffer::new(),
        };

        html.add_html();
        html.add_head(&mustgather)?;
        html.add_body(&mustgather)?;

        Ok(html)
    }
}

impl Html {
    pub fn render(self) -> String {
        String::from(self.buffer.finish())
    }
}

impl Html {
    fn add_body(&mut self, mustgather: &MustGather) -> Result<()> {
        let mut html = self.buffer.html();
        let mut body = html.body();

        // main div
        let mut app = body
            .div()
            .attr("id=\"app\"")
            .attr("class=\"container-fluid\"");
        let mut row = app.div().attr("class=\"row mt-2\"");
        let mut nav = row
            .div()
            .attr("class=\"col-2\"")
            .attr("id=\"nav-col\"")
            .div()
            .attr("class=\"list-group\"");
        row.div()
            .attr("class=\"col-10\"")
            .div()
            .attr("id=\"main-content\"")
            .attr("class=\"overflow-auto\"")
            .span()
            .attr("v-html=\"content\"");

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

    fn add_head(&mut self, mustgather: &MustGather) -> Result<()> {
        let mut html = self.buffer.html();
        let mut head = html.head();
        writeln!(head.title(), "{}", mustgather.title())?;
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

    fn add_html(&mut self) -> Result<()> {
        self.buffer.html().attr("lang='en'");
        Ok(())
    }
}
