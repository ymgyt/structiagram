use crate::errors::Error;
use crate::parse::Ast;
use std::collections::HashMap;
use syn::ItemStruct;

pub type Nodes = HashMap<String, ItemStruct>;
pub type Identifier = String;

pub struct Graph {
    pub edges: Vec<Edge>,
    pub nodes: Nodes,
}

#[derive(Debug)]
pub struct Edge {
    pub label: String,
    pub from: Identifier,
    pub to: Identifier,
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

        let mut edges = Vec::new();
        for (_, item) in &scope {
            edges.extend(Graph::find_edges(item, &scope));
        }

        Ok(Graph {
            edges,
            nodes: scope,
        })
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
                            match &last_seg.arguments {
                                syn::PathArguments::None => {
                                    let last_seg = last_seg.ident.to_string();
                                    if let Some(edge_type_ident) = last_seg.strip_suffix("Id") {
                                        if let Some(_edge_type) = scope.get(edge_type_ident) {
                                            acc.push(Edge {
                                                label: field
                                                    .ident
                                                    .as_ref()
                                                    .map(|ident| ident.to_string())
                                                    .unwrap_or_default(),
                                                from: ast.ident.to_string(),
                                                to: edge_type_ident.to_string(),
                                            });
                                        }
                                    }
                                }
                                syn::PathArguments::AngleBracketed(args) => {
                                    // Could eliminate duplicate code.
                                    if let Some(syn::GenericArgument::Type(syn::Type::Path(
                                        type_path,
                                    ))) = args.args.last()
                                    {
                                        if let Some(last_seg) = type_path.path.segments.last() {
                                            let last_seg = last_seg.ident.to_string();
                                            if let Some(edge_type_ident) =
                                                last_seg.strip_suffix("Id")
                                            {
                                                if let Some(_edge_type) = scope.get(edge_type_ident)
                                                {
                                                    // TODO: check if last_seg type is a collection
                                                    acc.push(Edge {
                                                        label: field
                                                            .ident
                                                            .as_ref()
                                                            .map(|ident| ident.to_string())
                                                            .unwrap_or_default(),
                                                        from: ast.ident.to_string(),
                                                        to: edge_type_ident.to_string(),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                syn::PathArguments::Parenthesized(_) => (), // Currently not supported.
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
