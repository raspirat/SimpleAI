pub mod compiler;

use crate::compiler::model::generate_model;
use serde_json::Value;

fn read_sgraph(file_path: &str) -> Value {
    let file = std::fs::File::open(file_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let model_graph: Value = serde_json::from_reader(reader).unwrap();
    model_graph
}

fn main() {
    let model_graph = read_sgraph("tests/mlp.sgraph");
    let res = generate_model(model_graph, "tests/mlp.py".to_string());
    if res.is_err() {
        println!("Error: {:?}", res.err().unwrap());
    }
}
