use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
// -------------------- ENVIRONEMENT -------------------- //
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub versions: Vec<String>,
    pub lib: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    pub deps: Vec<Dependency>,
}

impl Environment {
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let mut deps_sorted = self.deps.clone();
        deps_sorted.sort_by(|a, b| a.name.cmp(&b.name));

        for dep in &deps_sorted {
            hasher.update(dep.name.as_bytes());
            hasher.update(dep.lib.to_string().as_bytes());
            let mut versions_sorted = dep.versions.clone();
            versions_sorted.sort();
            for version in versions_sorted {
                hasher.update(version.as_bytes());
            }
        }

        format!("{:x}", hasher.finalize())
    }

    pub fn merge(&self, other: &Environment) -> Result<Environment, String> {
        let mut merged_deps: HashMap<(String, bool), Vec<String>> = HashMap::new();

        for dep in &self.deps {
            merged_deps.insert((dep.name.clone(), dep.lib), dep.versions.clone());
        }

        for dep in &other.deps {
            let key = (dep.name.clone(), dep.lib);
            if let Some(existing_versions) = merged_deps.get(&key) {
                let common_versions: Vec<String> = existing_versions
                    .iter()
                    .filter(|v| dep.versions.contains(v))
                    .cloned()
                    .collect();
                if common_versions.is_empty() {
                    return Err(format!(
                        "No compatible versions for dependency: {}",
                        dep.name
                    ));
                }
                merged_deps.insert(key, common_versions);
            } else {
                merged_deps.insert(key, dep.versions.clone());
            }
        }

        let merged_env = Environment {
            deps: merged_deps
                .into_iter()
                .map(|((name, lib), versions)| Dependency {
                    name,
                    versions,
                    lib,
                })
                .collect(),
        };

        Ok(merged_env)
    }
}
