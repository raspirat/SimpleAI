use crate::util::read_json_file;
use jsonschema::JSONSchema;
use serde_json::Value;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

fn collect_dependencies(
    node_name: &str,
    visited: &mut HashSet<String>,
) -> std::io::Result<HashSet<String>> {
    let mut dependencies = HashSet::new();

    if visited.contains(node_name) {
        return Ok(dependencies);
    }

    visited.insert(node_name.to_string());

    let node_path = format!("nodes/{}.json", node_name);
    println!("Node name: {:?}", node_path);
    let node_json = read_json_file(Path::new(&node_path))?;

    if let Some(deps) = node_json["dependencies"].as_array() {
        for dep in deps {
            if let Some(dep_str) = dep.as_str() {
                dependencies.insert(dep_str.to_string());
            }
        }
    }

    if let Some(forward) = node_json["forward"].as_array() {
        for forward_step in forward {
            if let Some(forward_object) = forward_step["object"].as_str() {
                let sub_dependencies = collect_dependencies(forward_object, visited)?;
                dependencies.extend(sub_dependencies);
            }
        }
    }

    Ok(dependencies)
}

fn make_imports(model_graph: Value, file: &mut File) -> std::io::Result<()> {
    let mut visited = HashSet::new();
    let mut all_dependencies = HashSet::new();

    if let Some(deps) = model_graph["dependencies"].as_array() {
        for dep in deps {
            if let Some(dep_str) = dep.as_str() {
                all_dependencies.insert(dep_str.to_string());
            }
        }
    }

    if let Some(forward) = model_graph["forward"].as_array() {
        for forward_step in forward {
            if let Some(forward_object) = forward_step["object"].as_str() {
                let dependencies = collect_dependencies(forward_object, &mut visited)?;
                all_dependencies.extend(dependencies);
            }
        }
    }

    println!("Deps: {:?}", all_dependencies);

    let mut imports = String::new();
    for dependency in all_dependencies {
        imports.push_str(&format!("import {}\n", dependency));
    }

    file.write_all(imports.as_bytes())?;
    Ok(())
}

fn make_model(model_graph: Value, file: &mut File) -> std::io::Result<()> {
    let mut model = String::new();

    // Class definition
    model.push_str(&format!(
        "\nclass {}(nn.Module):\n",
        model_graph["title"].as_str().unwrap()
    ));
    model.push_str("    def __init__(self");

    // Define constructor arguments
    if let Some(args) = model_graph["args"].as_array() {
        for arg in args {
            if let Some(arg_str) = arg["id"].as_str() {
                model.push_str(&format!(", {}", arg_str));
            }
            if let Some(display) = arg["object"]["display"].as_str() {
                model.push_str(&format!(": {}", display));
            }
        }
    }
    model.push_str("):\n");
    model.push_str("        super().__init__()\n");

    // Initialization for needs-init and attribute definitions
    if let Some(init) = model_graph["init"].as_array() {
        for init_step in init {
            if let Some(object) = init_step["object"].as_str() {
                if let Some(value) = init_step.get("value") {
                    model.push_str(&format!(
                        "        self.{} = {}\n",
                        object,
                        value.as_str().unwrap()
                    ));
                } else if let Some(code) = init_step.get("code") {
                    model.push_str(&format!("        {}\n", code.as_str().unwrap()));
                } else if let Some(forward) = model_graph["forward"].as_array() {
                    for forward_step in forward {
                        if let Some(id) = forward_step["id"].as_str() {
                            if id == object {
                                if let Some(object) = forward_step["object"].as_str() {
                                    let node_path = format!("nodes/{}.json", object);
                                    let node_json = read_json_file(Path::new(&node_path))?;

                                    if let Some(needs_init) = node_json["needs-init"].as_bool() {
                                        if needs_init {
                                            lint(&node_json, "module.scm", &node_path)?;
                                            make_model(node_json.clone(), file)?;
                                        } else {
                                            lint(&node_json, "no-init.scm", &node_path)?;
                                        }
                                    } else {
                                        lint(&node_json, "module.scm", &node_path)?;
                                    }
                                    if let Some(construct) = node_json["construct"].as_str() {
                                        let mut construct_code = construct.to_string();
                                        for (key, value) in forward_step.as_object().unwrap() {
                                            construct_code = construct_code.replace(
                                                &format!("{{{}}}", key),
                                                &value
                                                    .to_string()
                                                    .replace("false", "False")
                                                    .replace("true", "True"),
                                            );
                                        }
                                        model.push_str(&format!(
                                            "        self.{} = {}\n",
                                            id, construct_code
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Forward pass definition
    model.push_str("\n    def forward(self");
    if let Some(inputs) = model_graph["inputs"].as_array() {
        for (i, input) in inputs.iter().enumerate() {
            model.push_str(&format!(", input_{}", i));
            if let Some(display) = input["object"]["display"].as_str() {
                model.push_str(&format!(": {}", display));
            }
        }
    }
    model.push_str(") -> ");
    if let Some(outputs) = model_graph["outputs"].as_array() {
        if outputs.len() == 1 {
            model.push_str(outputs[0]["object"]["display"].as_str().unwrap());
        } else {
            model.push_str("tuple[");
            for (i, output) in outputs.iter().enumerate() {
                if i == 0 {
                    model.push_str(output["object"]["display"].as_str().unwrap());
                } else {
                    model.push_str(&format!(
                        ", {}",
                        output["object"]["display"].as_str().unwrap()
                    ));
                }
            }
            model.push(']');
        }
    }
    model.push_str(":\n");

    if let Some(forward) = model_graph["forward"].as_array() {
        for forward_step in forward {
            if let Some(input) = forward_step["input"].as_str() {
                if let Some(object) = forward_step["object"].as_str() {
                    let node_path = format!("nodes/{}.json", object);
                    let node_json = read_json_file(Path::new(&node_path))?;
                    // println!("Lint time!");
                    lint(&node_json, "module.scm", &node_path)?;

                    if let Some(forward_code) = node_json["usage"].as_array() {
                        for code in forward_code {
                            if let Some(code_str) = code["code"].as_str() {
                                let mut formatted_code = code_str.to_string();
                                if let Some(id) = forward_step["id"].as_str() {
                                    formatted_code =
                                        formatted_code.replace("{self}", &format!("self.{}", id));
                                }
                                formatted_code =
                                    formatted_code.replace("{input}", &input.replace('-', "_"));
                                for (key, value) in forward_step.as_object().unwrap() {
                                    formatted_code = formatted_code.replace(
                                        &format!("{{{}}}", key),
                                        &value
                                            .to_string()
                                            .replace("false", "False")
                                            .replace("true", "True"),
                                    );
                                }
                                if let Some(output) = forward_step["output"].as_str() {
                                    model.push_str(&format!(
                                        "        {} = {}\n",
                                        output.replace('-', "_"),
                                        formatted_code
                                    ));
                                } else {
                                    model.push_str(&format!("        {}\n", formatted_code));
                                }
                            }
                        }
                    }
                }
            } else if let Some(code) = forward_step["code"].as_str() {
                model.push_str(&format!("        {}\n", code));
            }
        }
    }

    if let Some(outputs) = model_graph["outputs"].as_array() {
        model.push_str("\n        return ");
        for i in 0..outputs.len() {
            if i == 0 {
                model.push_str(&format!("output_{}", i));
            } else {
                model.push_str(&format!(", output_{}", i));
            }
        }
        model.push('\n');
    }

    // Write the model definition to file
    file.write_all(model.as_bytes())?;
    Ok(())
}

fn load_schema(schema_name: &str) -> std::io::Result<JSONSchema> {
    let schema_path = Path::new("schemas").join(schema_name);
    let schema_content = fs::read_to_string(schema_path)?;
    let schema_json: Value = serde_json::from_str(&schema_content)?;
    let schema = JSONSchema::compile(&schema_json).unwrap();

    Ok(schema)
}

fn validate_json(json: &Value, schema: &JSONSchema) -> Result<(), Vec<String>> {
    let validation = schema.validate(json);
    let mut errors = Vec::new();
    if let Err(err) = validation {
        for error in err {
            errors.push(format!("{} ({})", error, error.instance_path).replace("()", "(/)"));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn lint(json: &Value, schema_name: &str, display_name: &str) -> std::io::Result<()> {
    let schema = load_schema(schema_name)?;
    match validate_json(json, &schema) {
        Ok(_) => {
            println!("{}: OK", display_name);
        }
        Err(errors) => {
            println!("{}: Errors:", display_name);
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
    Ok(())
}

pub fn generate_model(model_graph: Value, save_path: String) -> std::io::Result<()> {
    let mut model_file = File::create(save_path)?;
    lint(&model_graph, "module.scm", "Model")?;
    make_imports(model_graph.clone(), &mut model_file)?;
    make_model(model_graph.clone(), &mut model_file)?;

    Ok(())
}
