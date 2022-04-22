use camino::Utf8PathBuf;
use std::fs::File;
use std::io::{self, Write};

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

    /// Output file. default stdout.
    /// The '-' is interpreted as stdout.
    #[clap(long, short = 'o')]
    pub output: Option<Utf8PathBuf>,
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

        let mut output = StructiagramOptions::new_output(&self.output).unwrap();

        app.run(&mut output)
    }

    fn new_output(output: &Option<Utf8PathBuf>) -> Result<Box<dyn Write>, Error> {
        let output: Box<dyn Write> = match output.as_ref().map(|path| path.as_str()) {
            Some("-") | None => Box::new(io::stdout()),
            Some(path) => {
                Box::new(File::open(path).map_err(|err| Error::open_output_file(path, err))?)
            }
        };
        Ok(output)
    }
}

struct StructiagramApp {
    root_dir: Utf8PathBuf,
    parser: Parser,
    render: Render,
}

impl StructiagramApp {
    fn run(self, writer: &mut dyn Write) -> Result<(), Error> {
        let asts = self.parser.parse_files(self.root_dir.as_path())?;

        let graph = Graph::from_asts(asts)?;

        self.render.render(writer, &graph)?;

        Ok(())
    }
}
