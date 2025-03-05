use super::prelude::*;
use chrono::Utc;
/// This function searches through all available Nodes and returns a NodeContainer containing all Nodes available for the inferred environment.
pub fn search(query: String) -> NodeContainer {
    //  TODO: implement search function here and add a node container parameter to infer the
    //  environment

    //  TODO: remove this:
    let mut c = NodeContainer::new();
    c.push(Node {
        name: "test_code_node".to_string(),
        params: vec![],
        kind: NodeKind::Code {
            code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
        },
        description: "A simple code node".to_string(),
        author: "Author".to_string(),
        compiled: None,
        environment: Environment { deps: vec![] },
        date: Utc::now(),
    });
    c
}
