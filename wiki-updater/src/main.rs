use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::Write,
};

use codebake::{ops::OPERATIONS as Operations, OperationArgType};
use serde::Serialize;
use tinytemplate::TinyTemplate;

const TEMPLATE: &str = "# { name }
{{ for op in ops }}
* **`{ op.name }`** - { op.description } [{ op.authors }]
{{ if op.arguments }}
  args:
{{ for arg in op.arguments }}
  * *`{ arg.name }`*: { arg.type_string }
{{ endfor }}
{{ endif }}
{{ endfor }}";

#[derive(Serialize, Clone)]
struct OperationData<'a> {
    name: &'a str,
    description: &'a str,
    authors: String,
    arguments: Vec<ArgumentData<'a>>,
}

#[derive(Serialize)]
struct CategoryData<'a> {
    name: &'a str,
    ops: Vec<OperationData<'a>>,
}

#[derive(Serialize, Clone)]
struct ArgumentData<'a> {
    name: &'a str,
    type_string: &'a str,
}

fn main() {
    let mut tt = TinyTemplate::new();
    tt.add_template("category", TEMPLATE).unwrap();
    let mut categories: HashMap<&str, Vec<OperationData>> = HashMap::new();
    let mut output = "Every operation in codebake is named in `kebab-case` and may take zero or more parameters. The operations below are listed by category.\n".to_string();
    let mut sorted: Vec<&str> = Vec::new();

    for op in Operations {
        let authors = op.authors.join(", ").to_string();
        let mut arguments: Vec<ArgumentData> = Vec::new();

        for (arg_name, arg_type) in op.arguments {
            let type_string = match arg_type {
                OperationArgType::Integer => "int",
                OperationArgType::String => "string",
            };

            let arg = ArgumentData {
                name: arg_name,
                type_string,
            };

            arguments.push(arg);
        }

        let op_data = OperationData {
            name: op.name,
            description: op.description,
            authors,
            arguments,
        };

        match categories.entry(op.category) {
            Entry::Vacant(e) => {
                e.insert(vec![op_data]);

                sorted.push(op.category);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(op_data);
            }
        }
    }

    sorted.sort();

    for name in sorted {
        let category = CategoryData {
            name,
            ops: categories[name].clone(),
        };
        let category_output = &tt.render("category", &category).unwrap();

        output = format!("{}\n\n{}", output, category_output);
    }

    let mut file = File::create("Operation-Reference.md").unwrap();

    file.write(output.as_bytes()).unwrap();
}
