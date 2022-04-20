mod cli;
mod errors;
mod graph;
mod parse;
mod render;

fn main() {
    let app = cli::StructiagramOptions::parse();
    if let Err(err) = app.exec() {
        eprintln!("{err}");
    }
}
