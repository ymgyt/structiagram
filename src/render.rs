mod formatter;
use formatter::{IdentFormatter, TypeFormatter};

use crate::errors::Error;
use crate::graph::Graph;
use std::io::{self, Write};

pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Render {}
    }

    pub fn render(&self, writer: impl Write, graph: &Graph) -> Result<(), Error> {
        self.do_render(writer, graph).map_err(Error::render)
    }

    fn do_render(&self, mut writer: impl Write, graph: &Graph) -> Result<(), io::Error> {
        writeln!(&mut writer, "# Structiagram\n")?;
        writeln!(&mut writer, "```mermaid")?;
        writeln!(
            &mut writer,
            r#"%%{{init: {{
            "er": {{
                "layoutDirection": "LR",
                "entityPadding": 15,
                "useMaxWidth": false
            }}
        }}}}%%"#
        )?;
        writeln!(&mut writer, "erDiagram")?;

        self.render_entity_definition(&mut writer, graph)?;
        self.render_entity_relation(&mut writer, graph)?;

        writeln!(&mut writer, "```")?;

        Ok(())
    }

    fn render_entity_definition(
        &self,
        mut writer: impl Write,
        graph: &Graph,
    ) -> Result<(), io::Error> {
        for (ident, item_struct) in &graph.nodes {
            writeln!(writer, "{ident} {{")?;

            for field in &item_struct.fields {
                // TODO: render field type.
                writeln!(
                    writer,
                    "    {} {}",
                    TypeFormatter::new(&field.ty),
                    IdentFormatter::from(field.ident.as_ref()),
                )?;
            }

            writeln!(writer, "}}")?;
        }

        Ok(())
    }

    fn render_entity_relation(
        &self,
        mut writer: impl Write,
        graph: &Graph,
    ) -> Result<(), io::Error> {
        for edge in &graph.edges {
            writeln!(writer, "{} ||--|| {} : {}", edge.from, edge.to, edge.label)?;
        }

        Ok(())
    }
}
