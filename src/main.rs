pub mod compiler;
pub mod util;

use std::path::Path;
use std::process::exit;

use crate::compiler::model::generate_model;

fn main() {
    let model_graph = util::read_json_file(Path::new("tests/mlp.sgraph"));
    if model_graph.is_err() {
        println!("Error: {:?}", model_graph.err().unwrap());
        exit(-1);
    }
    let res = generate_model(model_graph.unwrap(), "tests/mlp.py".to_string());
    if res.is_err() {
        println!("Error: {:?}", res.err().unwrap());
        exit(-1);
    }
}
