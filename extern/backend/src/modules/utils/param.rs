use super::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
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
        connection: Option<Connection>,
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
