use camino::Utf8PathBuf;

use crate::errors::Error;
use crate::graph::Graph;
use crate::parse::Parser;
use crate::render::Render;

#[derive(clap::Parser, Debug)]
#[clap(version, propagate_version = true)]
pub struct StructiagramOptions {
    /// Root directory to parse files.
    #[clap(long)]
    pub dir: Utf8PathBuf,
}

impl StructiagramOptions {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }

    pub fn exec(self) -> Result<(), Error> {
        let app = StructiagramApp {
            root_dir: self.dir,
            parser: Parser::new(),
            render: Render::new(),
        };

        app.run()
    }
}

struct StructiagramApp {
    root_dir: Utf8PathBuf,
    parser: Parser,
    render: Render,
}

impl StructiagramApp {
    fn run(self) -> Result<(), Error> {
        let asts = self.parser.parse_files(self.root_dir.as_path())?;

        let graph = Graph::from_asts(asts)?;

        self.render.render(&graph)?;

        Ok(())
    }
}
