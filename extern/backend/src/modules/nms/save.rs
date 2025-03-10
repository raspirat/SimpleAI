use crate::nms::check_name;
use crate::utils::prelude::{save::*, *};
use anyhow::Result;
use std::fs::{create_dir, create_dir_all, File};
use std::io::Write;
use std::path::Path;

// #[cfg(feature = "desktop")]
pub fn save_node(node: Node) -> Result<(), String> {
    let name = node.name.clone();

    if !check_name(name.clone()) {
        return Err(format!(
            "Node name {} is not allowed! Please only use letters, dashes and underscores.",
            name
        ));
    }

    let node_path = Path::new("nodes/").join(&name);
    if !node_path.exists() {
        create_dir_all(&node_path).map_err(|e| e.to_string())?;
    }

    let meta_path = node_path.join("meta.toml");
    let mut meta: Metadata = if meta_path.exists() {
        let meta_content = std::fs::read_to_string(&meta_path).map_err(|e| e.to_string())?;
        toml::from_str(&meta_content).map_err(|e| e.to_string())?
    } else {
        Metadata::from(node.clone())
    };

    let new_version = Version {
        version: node.version.version.clone(),
        env: node.clone().get_full_env(),
    };

    if !meta
        .versions
        .iter()
        .any(|v| v.version == new_version.version)
    {
        meta.versions.push(new_version.clone());
    }

    let meta_toml = toml::to_string(&meta).unwrap();
    let mut meta_file = File::create(&meta_path).map_err(|e| e.to_string())?;
    meta_file
        .write_all(meta_toml.as_bytes())
        .map_err(|e| e.to_string())?;

    let env_hash = new_version.env.hash();
    let env_path = node_path.join(&env_hash);
    if !env_path.exists() {
        create_dir(&env_path).map_err(|e| e.to_string())?;
    }

    let bin_path = env_path.join(format!("{}.bin", new_version.version));

    let node_bin = bincode::serialize(&SaveNode::from(node)).map_err(|e| e.to_string())?;
    let mut node_file = File::create(bin_path).map_err(|e| e.to_string())?;
    node_file.write_all(&node_bin).map_err(|e| e.to_string())?;

    Ok(())
}
