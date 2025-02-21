// this should be replaced by the backend Node
#[derive(Clone, PartialEq)]
pub(crate) enum NodeParam {
    Named { name: String },
}

#[derive(Clone, PartialEq)]
pub(crate) struct Node {
    pub name: String,
    pub params: Vec<NodeParam>,
}
