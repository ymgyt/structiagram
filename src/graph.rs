use crate::errors::Error;
use crate::parse::Ast;
use std::collections::HashMap;
use syn::{Ident, ItemStruct};

pub type Scope = HashMap<String, ItemStruct>;

pub struct Graph {
    pub edges: Vec<Edge>,
    pub scope: Scope,
}

#[derive(Debug)]
pub struct Edge {
    pub from: ItemStruct,
    pub to: ItemStruct,
}

impl Graph {
    pub fn from_asts(asts: impl IntoIterator<Item = Ast>) -> Result<Self, Error> {
        let scope = asts.into_iter().fold(HashMap::new(), |mut acc, ast| {
            match ast {
                Ast::Struct(item) => {
                    acc.insert(item.ident.to_string(), item);
                }
            }
            acc
        });

        //println!("{:#?}", scope);

        let mut edges = Vec::new();
        for (_, item) in &scope {
            edges.extend(Graph::find_edges(item, &scope));
        }

        //println!("{:#?}", edges);

        Ok(Graph { edges, scope })
    }

    fn find_edges(
        ast: &ItemStruct,
        scope: &HashMap<String, ItemStruct>,
    ) -> impl Iterator<Item = Edge> {
        ast.fields
            .iter()
            .fold(Vec::new(), |mut acc, field| {
                // Ignore self relation.
                if let Some(ident) = &field.ident {
                    if ident.to_string() == "id" {
                        return acc;
                    }
                }
                // Construct userId: models::UserId => (ast -> User) edge.
                match field.ty {
                    syn::Type::Path(ref type_path) => {
                        if let Some(last_seg) = type_path.path.segments.last() {
                            let last_seg = last_seg.ident.to_string();
                            if let Some(edge_type) = last_seg.strip_suffix("Id") {
                                if let Some(edge_type) = scope.get(edge_type) {
                                    acc.push(Edge {
                                        from: ast.clone(),
                                        to: edge_type.clone(),
                                    });
                                }
                            }
                        }
                    }
                    _ => (),
                }
                acc
            })
            .into_iter()
    }
}
