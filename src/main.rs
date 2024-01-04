#![allow(unused)]
use std::fs::File;

use tree_sitter::{Node, Parser, Tree};

fn match_type(t: String) -> String {
    let matched = match t.as_str() {
        "i32" => "int",
        "f32" => "float",
        "string" => "char *",
        "bool" => "int",
        _ => "",
    };

    matched.to_owned()
}

fn get_field(src: &str, node: &Node, name: &str) -> String {
    let child_node = node.child_by_field_name(name).unwrap();
    child_node.utf8_text(src.as_bytes()).unwrap().to_string()
}

fn translate_to_c(src: &str, tree: &Tree) -> String {
    let root = tree.root_node();
    let mut cursor = root.walk();
    let mut c_code = String::new();

    for child in root.children(&mut cursor) {
        match child.kind() {
            "function_definition" => {
                let translated_function = translate_function(src, &child);
                c_code.push_str(&translated_function);
            }

            "variable_declaration" => {
                let translated_variable = translate_variable_declaration(src, &child);
                c_code.push_str(&translated_variable);
            }
            _ => {}
        }
    }

    c_code
}

fn translate_variable_declaration(src: &str, node: &tree_sitter::Node) -> String {
    let var_type = match_type(get_field(src, node, "type"));
    let var_name = get_field(src, node, "name");
    let mutability = match get_field(src, node, "mutability_specifier").as_str() {
        "const" => "const ",
        _ => ""
    }.to_owned();

    let var_value = node.child_by_field_name("value").unwrap();

    let mut c_code = format!("{}{} {} = {};\n", mutability, var_type, var_name, var_value.utf8_text(src.as_bytes()).unwrap());

    c_code
}

fn translate_function(src: &str, node: &Node) -> String {
    let function_name = get_field(src, node, "name");
    let return_type = match_type(get_field(src, node, "return_type"));
    let parameters = node.child_by_field_name("parameters").unwrap();
    let body = node.child_by_field_name("body").unwrap();

    let mut c_code = format!("{} {}(", return_type, function_name);


    if parameters.child_count() > 0 {
        let mut param_list = String::new();
        let mut cursor = node.walk();
        for child in parameters.children(&mut cursor) {
            if child.kind() == "parameter" {
                let param_name = get_field(src, &child, "name");
                let param_type = match_type(get_field(src, &child, "type"));

                param_list.push_str(&format!("{} {}, ", param_type, param_name));
            }
        }
        param_list.pop(); // Remove the trailing comma
        param_list.pop(); // Remove the space
        c_code.push_str(&param_list);
    }

    c_code.push_str(") {\n");

    let body_code = translate_block(src, &body);
    c_code.push_str(&body_code);

    c_code.push_str("}\n");

    c_code
}

fn translate_block(src: &str, node: &tree_sitter::Node) -> String {
    let mut c_code = String::new();
    let mut cursor = node.walk();

    for statement in node.children(&mut cursor) {
        let statement_code = match statement.kind() {
            "return_statement" => translate_return_statement(src, &statement),
            "variable_declaration" => translate_variable_declaration(src, &statement),
            "assignment_statement" => {
                let text = statement.utf8_text(src.as_bytes()).unwrap().to_string();
                format!("{text}\n")
            }
            "while_statement" => {
                let condition = get_field(src, &statement, "condition");
                let body = statement.child_by_field_name("body").unwrap();
                let body_text = translate_block(src, &body);

                format!("while ({condition}) {{\n{body_text}}}\n")
            }
            "if_statement" => {
                let condition = get_field(src, &statement, "condition");
                let body = statement.child_by_field_name("body").unwrap();
                let body_text = translate_block(src, &body);

                format!("if ({condition}) {{\n{body_text}}}\n")
            }
            _ => String::new(),
        };

        c_code.push_str(&statement_code);
    }

    c_code
}

fn translate_return_statement(src: &str, node: &Node) -> String {
    let child_node = node.child(1).unwrap();
    let return_expr_code = child_node.utf8_text(src.as_bytes()).unwrap().to_string();

    format!("return {};\n", return_expr_code)
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let in_file = &args[1];
    let out_file = &args[2];

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_miac::language())
        .expect("Error loading Miac grammar");

    let src = std::fs::read_to_string(in_file).expect("Failed to open input file");

    let tree = parser.parse(src.clone(), None).unwrap();

    let c_code = translate_to_c(&src, &tree);
    std::fs::write(out_file, c_code);

    Ok(())
}
