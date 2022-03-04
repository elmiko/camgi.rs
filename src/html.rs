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
            .write_str(include_str!("../templates/style.css"))?;
        Ok(())
    }

    fn add_html(&mut self) -> Result<()> {
        self.buffer.html().attr("lang='en'");
        Ok(())
    }
}
