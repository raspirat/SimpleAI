use super::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SaveParamKind {
    Runtime {
        kind: RuntimeParamKind,
        connected_to: Option<u128>,
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

impl From<ParamKind> for SaveParamKind {
    fn from(param_kind: ParamKind) -> Self {
        if let ParamKind::Static { value } = param_kind {
            SaveParamKind::Static { value }
        } else {
            match param_kind {
                ParamKind::Runtime {
                    kind,
                    connection,
                    id: _,
                } => {
                    if let Some(connected) = connection {
                        let connected_param = connected
                            .context
                            .upgrade()
                            .unwrap()
                            .try_lock()
                            .unwrap()
                            .to_owned();

                        if let ParamKind::Runtime {
                            kind: _,
                            connection: _,
                            id,
                        } = connected_param.kind
                        {
                            SaveParamKind::Runtime {
                                kind,
                                connected_to: Some(id),
                            }
                        } else {
                            panic!("A parameter must be connected with a runtime parameter!");
                        }
                    } else {
                        SaveParamKind::Runtime {
                            kind,
                            connected_to: None,
                        }
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
        let param = strong_param.context.try_lock().unwrap().to_owned();
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
