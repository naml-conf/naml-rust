use crate::lexer::naml_lexer::NamlLexer;
use crate::lexer::naml_token::NamlToken::{AssignmentOperator, BlockClose, BlockOpen, Text};

#[test]
fn test_lexer() {
    let file = "example {
int = 3
double = 3.2
boolean = y
  child {
    string = \"my_string\"
  }
}";

    let lexer_result = NamlLexer::tokenize(&file.to_string().into_bytes());
    assert_eq!(false, lexer_result.is_err());

    let expected = vec![
        Text("example".to_string()),
        BlockOpen,
        Text("int".to_string()),
        AssignmentOperator,
        Text("3".to_string()),
        Text("double".to_string()),
        AssignmentOperator,
        Text("3.2".to_string()),
        Text("boolean".to_string()),
        AssignmentOperator,
        Text("y".to_string()),
        Text("child".to_string()),
        BlockOpen,
        Text("string".to_string()),
        AssignmentOperator,
        Text("\"my_string\"".to_string()),
        BlockClose,
        BlockClose,
    ];
    let tokens = lexer_result.ok().unwrap();

    for (i, token) in expected.iter().enumerate() {
        let found_token = tokens.get(i);
        assert_eq!(true, found_token.is_some());
        assert_eq!(token, found_token.unwrap());
    }
}
