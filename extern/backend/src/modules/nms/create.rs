use crate::nms::check_name;
use crate::utils::prelude::{save::*, *};
use anyhow::Result;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

// #[cfg(feature = "desktop")]
pub fn create_node(node: Node) -> Result<(), String> {
    let name = node.name.clone();

    if !check_name(name.clone()) {
        return Err(format!(
            "Node name {} is not allowed! Please only use letters, dashes and underscores.",
            name
        ));
    }
    if !Path::new("nodes/").exists() {
        create_dir(Path::new("nodes/")).map_err(|e| e.to_string())?;
    }
    if Path::new("nodes/").join(name.clone()).exists() {
        return Err(format!("A node named {} does already exist!", name));
    }

    create_dir(Path::new("nodes/").join(name.clone())).map_err(|e| e.to_string())?;
    let meta: Metadata = node.clone().into();
    let meta_toml = toml::to_string(&meta).unwrap();
    let mut meta_file = File::create(Path::new("nodes/").join(name.clone()).join("meta.toml"))
        .map_err(|e| e.to_string())?;
    meta_file
        .write_all(meta_toml.as_bytes())
        .map_err(|e| e.to_string())?;

    let env_hash = meta.impls[0].clone().1;
    create_dir(
        Path::new("nodes/")
            .join(name.clone())
            .join(env_hash.clone()),
    )
    .map_err(|e| e.to_string())?;
    let node_bin = bincode::serialize(&SaveNode::from(node)).map_err(|e| e.to_string())?;
    let mut node_file = File::create(
        Path::new("nodes/")
            .join(name)
            .join(env_hash)
            .join("node.bin"),
    )
    .map_err(|e| e.to_string())?;
    node_file.write_all(&node_bin).map_err(|e| e.to_string())?;

    Ok(())
}
