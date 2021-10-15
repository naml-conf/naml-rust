/// The naml token enum represents all possible tokens defined by the language grammar.
#[derive(PartialEq, Debug)]
pub enum NamlToken {
    BlockOpen,
    BlockClose,
    Text(String),
    AssignmentOperator,
}

impl ToString for NamlToken {
    fn to_string(&self) -> String {
        match self {
            NamlToken::BlockOpen => "{".to_string(),
            NamlToken::BlockClose => "}".to_string(),
            NamlToken::Text(t) => format!("text({})", t),
            NamlToken::AssignmentOperator => "=".to_string(),
        }
    }
}
