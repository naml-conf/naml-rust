
use crate::lexer::naml_lexer::NamlLexer;
use crate::naml_node::{NamlNodeElement, NamlValue};

use crate::parser::naml_parser::NamlParser;

#[test]
fn test_parser() {
    let file = "example {
int = 3
double = 3.2
boolean = y
  child {
    string = \"my_string\"
  }
}";

    let lexer_result = NamlLexer::tokenize(&file.to_string().into_bytes());
    assert_eq!(true, lexer_result.is_ok());

    let mut tokens = lexer_result.ok().expect("Impossible");

    let parser_result = NamlParser::parse_tokens(&mut tokens);
    if let Err(e) = parser_result {
        println!("{:?}", e);
        return;
    }
    assert_eq!(true, parser_result.is_ok());

    let document = parser_result.ok().expect("Impossible");
    print_node(&document, "root".to_string())
}

fn print_node(node: &NamlNodeElement, path: String) {
    for child in node.children() {
        let var_path = format!("{}.{}", path, child.to_owned());
        match node.child(child.to_owned()).expect("Impossible") {
            NamlValue::String(s) => println!("string({}): {}", var_path, s),
            NamlValue::Integer(s) => println!("integer({}): {}", var_path, s),
            NamlValue::Double(s) => println!("double({}): {}", var_path, s),
            NamlValue::Boolean(s) => println!("boolean({}): {}", var_path, s),
            NamlValue::NamlNode(n) => {
                print_node(n, var_path)
            }
        }
    }
}
