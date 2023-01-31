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
        let mut nodes: Vec<(&String, &syn::ItemStruct)> = graph.nodes.iter().collect();
        nodes.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (ident, item_struct) in nodes {
            writeln!(writer, "{ident} {{")?;

            // Skip unnamed fields.
            // https://github.com/ymgyt/structiagram/issues/3
            if let syn::Fields::Named(_) = &item_struct.fields {
                for field in &item_struct.fields {
                    writeln!(
                        writer,
                        "    {} {}",
                        TypeFormatter::new(&field.ty),
                        IdentFormatter::from(field.ident.as_ref()),
                    )?;
                }
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
        let mut edges: Vec<_> = graph.edges.iter().collect();
        edges.sort_by(|a, b| {
            if a.from == b.from {
                a.to.cmp(&b.to)
            } else {
                a.from.cmp(&b.from)
            }
        });

        for edge in edges {
            writeln!(writer, "{} ||--|| {} : {}", edge.from, edge.to, edge.label)?;
        }

        Ok(())
    }
}
