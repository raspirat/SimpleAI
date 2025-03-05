use super::prelude::*;

// -------------------- CONTAINER -------------------- //
#[derive(Clone, PartialEq)]
pub struct Container<T> {
    pub tree: Vec<StrongContext<T>>,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self { tree: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.push_context(StrongContext::from(item))
    }
    pub fn push_context(&mut self, ctx: StrongContext<T>) {
        self.tree.push(ctx);
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
