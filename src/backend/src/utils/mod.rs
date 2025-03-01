use std::{collections::HashMap, sync::*};

use derive_builder::Builder;
use derive_new::new;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

// -------------------- DATE -------------------- //
pub type Date = DateTime<Utc>;

// -------------------- STRONG CONTEXT -------------------- //
#[derive(new, Clone)]
pub struct StrongContext<T> {
    pub context: Arc<Mutex<T>>,
}

impl<T> StrongContext<T> {
    pub fn downgrade(self) -> WeakContext<T> {
        WeakContext::from(self)
    }
}

impl<T> PartialEq for StrongContext<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if let Ok(self_data) = self.context.lock() {
            if let Ok(other_data) = other.context.lock() {
                return *self_data == *other_data;
            }
        }
        false
    }
}

impl<T> From<T> for StrongContext<T> {
    fn from(t: T) -> Self {
        Self::new(Arc::new(Mutex::new(t)))
    }
}

impl<T> From<WeakContext<T>> for Option<StrongContext<T>> {
    fn from(weak: WeakContext<T>) -> Option<StrongContext<T>> {
        if let Some(c) = weak.context.upgrade() {
            return Some(StrongContext::new(c));
        }
        None
    }
}

// -------------------- WEAK CONTEXT -------------------- //
#[derive(new, Clone)]
pub struct WeakContext<T> {
    context: Weak<Mutex<T>>,
}
impl<T> WeakContext<T> {
    fn upgrade(self) -> Option<StrongContext<T>> {
        Option::<StrongContext<T>>::from(self)
    }
}
impl<T> PartialEq for WeakContext<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if let Some(self_ctx) = self.context.upgrade() {
            if let Some(other_ctx) = other.context.upgrade() {
                if let Ok(self_data) = self_ctx.lock() {
                    if let Ok(other_data) = other_ctx.lock() {
                        return *self_data == *other_data;
                    }
                }
            }
        }
        false
    }
}
impl<T> From<StrongContext<T>> for WeakContext<T> {
    fn from(other: StrongContext<T>) -> Self {
        Self::new(Arc::downgrade(&other.context))
    }
}

// -------------------- CONTAINER -------------------- //
#[derive(Clone, PartialEq)]
pub struct Container<T> {
    tree: Vec<StrongContext<T>>,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self { tree: Vec::new() }
    }
    pub fn push_context(&mut self, node_ctx: StrongContext<T>) {
        self.tree.push(node_ctx);
    }
}

impl<T> Default for Container<T> {
    fn default() -> Self {
        Self::new()
    }
}

// -------------------- NODE CONTAINER -------------------- //
pub type NodeContainer = Container<Node>;

// -------------------- PARAM CONTAINER -------------------- //
pub type ParamContainer = Container<Param>;

// -------------------- DTYPE -------------------- //
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DType {
    Tensor {
        dtype: Box<DType>,
        shape: Vec<String>,
    },
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Void,
}

// -------------------- RUNTIME PARAM KIND -------------------- //
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeParamKind {
    Input,
    Output,
}

// -------------------- PARAM KIND -------------------- //
pub type Connection = WeakContext<Param>;
#[derive(Clone, PartialEq)]
pub enum ParamKind {
    Runtime {
        kind: RuntimeParamKind,
        connection: Connection,
        id: u128,
    },
    Static {
        value: String,
    },
}

// -------------------- PARAM  -------------------- //
pub type StrongParam = StrongContext<Param>;
pub type WeakParam = WeakContext<Param>;
#[derive(Builder, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub desc: String,
    pub kind: ParamKind,
    pub dtype: DType,
}

// -------------------- CODE NODE -------------------- //
#[derive(Builder, Clone, PartialEq)]
pub struct CodeNode {
    pub code: String,
}

// -------------------- BUNDLED NODE -------------------- //
#[derive(Builder, Clone, PartialEq)]
pub struct BundledNode {
    bundle: NodeContainer,
}

// -------------------- NODE KIND -------------------- //
#[derive(Clone, PartialEq)]
pub enum NodeKind {
    Code { code: String },
    Bundled { bundle: NodeContainer },
}

// -------------------- NODE -------------------- //
pub type StrongNode = StrongContext<Node>;
pub type WeakNode = WeakContext<Node>;

#[derive(Builder, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub params: Vec<StrongParam>,
    pub kind: NodeKind,
    pub description: String,
    pub author: String,
    pub compiled: Option<String>, // or and bytes...
    pub environment: Environment,
    pub date: Date,
}

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
            .name(node.name)
            .description(node.description)
            .author(node.author)
            .date(node.date)
            .impls(vec![(node.environment.clone(), node.environment.hash())])
            .build()
            .expect("Internal error")
    }
}

// -------------------- SAVE NODES -------------------- //
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SaveParamKind {
    Runtime {
        kind: RuntimeParamKind,
        connected_to: u128,
    },
    Static {
        value: String,
    },
}

#[derive(Builder, Clone, Serialize, Deserialize)]
pub struct SaveParam {
    pub name: String,
    pub desc: String,
    pub kind: SaveParamKind,
    pub dtype: DType,
    pub id: u128,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum SaveNodeKind {
    Code { code: String },
    Bundled { bundle: Vec<SaveNode> },
}

#[derive(Builder, Clone, Serialize, Deserialize)]
pub struct SaveNode {
    pub name: String,
    pub params: Vec<SaveParam>,
    pub kind: SaveNodeKind,
    pub description: String,
    pub author: String,
    pub compiled: Option<String>,
    pub environment: Environment,
    pub date: Date,
}

impl From<NodeContainer> for SaveNodeKind {
    fn from(nodes: NodeContainer) -> Self {
        let mut save_nodes: Vec<SaveNode> = Vec::new();
        for context in nodes.tree.iter() {
            let node: Node = context.context.lock().unwrap().to_owned();
            save_nodes.push(node.into());
        }
        SaveNodeKind::Bundled { bundle: save_nodes }
    }
}

impl From<ParamKind> for SaveParamKind {
    fn from(param_kind: ParamKind) -> Self {
        if let ParamKind::Static { value } = param_kind {
            SaveParamKind::Static { value }
        } else {
            match param_kind {
                ParamKind::Runtime {
                    kind: _,
                    connection,
                    id: _,
                } => {
                    let connected_param = connection
                        .context
                        .upgrade()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .to_owned();

                    if let ParamKind::Runtime {
                        kind: connected_kind,
                        connection: _,
                        id,
                    } = connected_param.kind
                    {
                        SaveParamKind::Runtime {
                            kind: connected_kind,
                            connected_to: id,
                        }
                    } else {
                        panic!("A parameter must be connected with a runtime parameter!");
                    }
                }
                _ => {
                    panic!("An interal error has occured.");
                }
            }
        }
    }
}

impl From<StrongParam> for SaveParam {
    fn from(strong_param: StrongParam) -> Self {
        let param = strong_param.context.lock().unwrap().to_owned();
        let mut binding = SaveParamBuilder::default();
        let builder = binding
            .name(param.name)
            .desc(param.desc)
            .dtype(param.dtype)
            .id(Uuid::new_v4().as_u128())
            .kind(param.kind.into());

        builder.build().unwrap()
    }
}

impl From<Node> for SaveNode {
    fn from(node: Node) -> Self {
        let mut binding = SaveNodeBuilder::default();
        let mut builder = binding
            .name(node.name)
            .description(node.description)
            .author(node.author)
            .compiled(node.compiled)
            .environment(node.environment)
            .date(node.date);

        if let NodeKind::Code { code } = node.kind {
            builder = builder.kind(SaveNodeKind::Code { code });
        } else if let NodeKind::Bundled { bundle } = node.kind {
            builder = builder.kind(bundle.into());
        }

        builder = builder.params(
            node.params
                .iter()
                .map(|param| SaveParam::from(param.clone()))
                .collect(),
        );

        builder.build().unwrap()
    }
}

impl From<SaveNode> for Node {
    fn from(node: SaveNode) -> Self {
        let mut param_map: HashMap<u128, StrongParam> = HashMap::new();
        let mut params = Vec::new();

        // First pass: instantiate Params without setting connections
        for save_param in &node.params {
            let strong_param = StrongParam::from(Param {
                name: save_param.name.clone(),
                desc: save_param.desc.clone(),
                dtype: save_param.dtype.clone(),
                kind: match &save_param.kind {
                    SaveParamKind::Static { value } => ParamKind::Static {
                        value: value.clone(),
                    },
                    SaveParamKind::Runtime { kind, .. } => ParamKind::Runtime {
                        kind: kind.clone(),
                        connection: WeakContext::new(Weak::new()), // Placeholder
                        id: save_param.id,
                    },
                },
            });
            param_map.insert(save_param.id, strong_param.clone());
            params.push(strong_param);
        }

        // Second pass: resolve connections
        for strong_param in &params {
            let mut param = strong_param.context.lock().unwrap();
            if let ParamKind::Runtime { connection, id, .. } = &mut param.kind {
                if let Some(target) = param_map.get(id) {
                    *connection = WeakContext::from(target.clone());
                }
            }
        }

        let mut binding = NodeBuilder::default();
        let builder = binding
            .name(node.name)
            .description(node.description)
            .author(node.author)
            .compiled(node.compiled)
            .environment(node.environment)
            .date(node.date)
            .params(params);

        let kind = match node.kind {
            SaveNodeKind::Code { code } => NodeKind::Code { code },
            SaveNodeKind::Bundled { bundle } => {
                let mut node_container = NodeContainer::new();
                for save_node in bundle {
                    node_container.push_context(StrongNode::from(Node::from(save_node)));
                }
                NodeKind::Bundled {
                    bundle: node_container,
                }
            }
        };

        builder.kind(kind).build().expect("Failed to build Node")
    }
}
