use super::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
// -------------------- METADATA -------------------- //
#[derive(Builder, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub author: String,
    pub date: Date,
    pub impls: Vec<(Environment, String)>,
}

impl From<Node> for Metadata {
    fn from(node: Node) -> Self {
        MetadataBuilder::default()
            .name(node.name.clone())
            .description(node.description.clone())
            .author(node.author.clone())
            .date(node.date)
            .impls(vec![(
                node.clone().get_full_env().clone(),
                node.clone().get_full_env().hash(),
            )])
            .build()
            .expect("Internal error")
    }
}
