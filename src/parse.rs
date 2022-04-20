use crate::errors::Error;
use camino::Utf8Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use walkdir::{DirEntry, WalkDir};

pub enum Ast {
    Struct(syn::ItemStruct),
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    /// Parse target structs by walking root directory.
    pub fn parse_files(&self, root_dir: impl AsRef<Utf8Path>) -> Result<Vec<Ast>, Error> {
        fn is_rust_file(entry: &DirEntry) -> bool {
            entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("rs"))
        }

        let r: Result<Vec<Ast>, Error> = WalkDir::new(root_dir.as_ref().as_std_path())
            .into_iter()
            .filter_map(|result| match result {
                Ok(ref entry) if is_rust_file(entry) => Some(result),
                Ok(_) => None,
                Err(_) => Some(result),
            })
            .try_fold(Vec::new(), |mut acc, entry| {
                let entry = entry?;
                // TODO: Handle convert error.
                acc.extend(self.parse_file(Utf8Path::from_path(entry.path()).unwrap())?);
                Ok(acc)
            });

        r
    }

    fn parse_file(&self, path: &Utf8Path) -> Result<impl Iterator<Item = Ast>, Error> {
        let mut file =
            File::open(path.as_std_path()).map_err(|err| Error::open_rust_file(path, err))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|err| Error::open_rust_file(path, err))?;

        self.parse_file_content(&content)
    }

    fn parse_file_content(&self, content: &str) -> Result<impl Iterator<Item = Ast>, Error> {
        // TODO: Handle syn error.
        let ast = syn::parse_file(content).unwrap();

        // TODO: check attribute like #[diagram]
        Ok(ast.items.into_iter().filter_map(|item| match item {
            syn::Item::Struct(item) => Some(Ast::Struct(item)),
            _ => None,
        }))
    }
}
