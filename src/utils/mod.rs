use std::sync::*;

use derive_builder::Builder;
use derive_new::new;

use chrono::prelude::*;

// -------------------- DATE -------------------- //
pub type Date = DateTime<Utc>;

// -------------------- STRONG CONTEXT -------------------- //
#[derive(new, Clone)]
pub struct StrongContext<T> {
    pub context: Arc<Mutex<T>>,
}
impl<T> StrongContext<T> {
    pub fn downgrade(&self) -> WeakContext<T> {
        WeakContext::from(*self)
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
    fn upgrade(&self) -> Option<StrongContext<T>> {
        Option::<StrongContext<T>>::from(*self)
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

// -------------------- RUNTIME PARAM KIND -------------------- //
#[derive(Clone, PartialEq)]
pub enum RuntimeParamKind {
    Input,
    Output,
}

// -------------------- RUNTIME PARAM -------------------- //
#[derive(Builder, Clone, PartialEq)]
pub struct RuntimeParam {
    kind: RuntimeParamKind,
    connection: Connection,
}

// -------------------- STATIC PARAM -------------------- //
#[derive(Builder, Clone, PartialEq)]
pub struct StaticParam {
    value: String, // or bytes
}

// -------------------- PARAM KIND -------------------- //
pub type Connection = WeakContext<ParamKind>;
#[derive(Clone, PartialEq)]
pub enum ParamKind {
    Runtime { runtime: RuntimeParam },
    Static { stat: StaticParam },
}

// -------------------- DTYPE -------------------- //
#[derive(Clone, PartialEq)]
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

// -------------------- PARAM CONTAINER -------------------- //
pub type ParamContainer = Container<Param>;

// -------------------- CODE NODE -------------------- //
#[derive(Builder, Clone, PartialEq)]
pub struct CodeNode {}

// -------------------- BUNDLED NODE -------------------- //
#[derive(Builder, Clone, PartialEq)]
pub struct BundledNode {
    bundle: NodeContainer,
}

// -------------------- NODE KIND -------------------- //
#[derive(Clone, PartialEq)]
pub enum NodeKind {
    Code(CodeNode),
    Bundled(BundledNode),
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
    pub date: Date,
}

// -------------------- NODE CONTAINER -------------------- //
pub type NodeContainer = Container<Node>;
