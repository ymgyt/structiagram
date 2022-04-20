use crate::errors::Error;
use crate::graph::Graph;
use quote::quote;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;

// TODO: Inject output writer.
pub struct Render {}

impl Render {
    pub fn new() -> Self {
        Render {}
    }

    pub fn render(&self, graph: &Graph) -> Result<(), Error> {
        // TODO: DO NOT write file
        let mut out = File::create("./structiagram.md").unwrap();

        let _content = r#"
        User {
            string id
            string name
        }
        Document {
            string id
            string created_by
        }
        Document ||--|| User : created_by
        "#;

        use std::io::Write;
        writeln!(&mut out, "# Structiagram\n");
        writeln!(&mut out, "```mermaid");
        writeln!(&mut out, "erDiagram");

        self.render_entity_definition(&mut out, graph)?;
        self.render_entity_relation(&mut out, graph)?;

        //writeln!(&mut out, "{content}");
        writeln!(&mut out, "```");

        Ok(())
    }

    fn render_entity_definition(&self, mut writer: impl Write, graph: &Graph) -> Result<(), Error> {
        for (ident, item_struct) in &graph.scope {
            writeln!(writer, "{ident} {{");

            for field in &item_struct.fields {
                // TODO: render field type.
                writeln!(writer, "    string {}", IdentFormatter(&field.ident),);
            }

            writeln!(writer, "}}");
        }

        Ok(())
    }

    fn render_entity_relation(&self, mut writer: impl Write, graph: &Graph) -> Result<(), Error> {
        for edge in &graph.edges {
            writeln!(
                writer,
                "{} ||--|| {} : TODO",
                IdentFormatter(&Some(edge.from.ident.clone())),
                IdentFormatter(&Some(edge.to.ident.clone())),
            );
        }

        Ok(())
    }
}

struct IdentFormatter<'a>(&'a Option<syn::Ident>);

impl<'a> fmt::Display for IdentFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .as_ref()
                .map(|ident| ident.to_string())
                .unwrap_or_default()
        )
    }
}
