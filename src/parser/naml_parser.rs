use std::ops::Deref;
use crate::lexer::naml_lexer::{NamlLexer, NamlLexerError};

use crate::lexer::naml_token::NamlToken;
use crate::lexer::naml_token::NamlToken::{BlockClose, Text};
use crate::naml_node::{NamlNodeElement, NamlValue};

use crate::parser::naml_parser::NamlParserError::{LexerError, SyntaxError, UnknownVariable, VariableParseError};

/// The naml parser is capable of parsing a stream of tokens from the naml lexer.
pub struct NamlParser;

impl NamlParser {
    /// Parses a given input vector of bytes into a naml node structure.
    pub fn parse(input: &Vec<u8>) -> Result<NamlNodeElement, NamlParserError> {
        NamlParser::parse_tokens(&mut NamlLexer::tokenize(input).map_err(|e| LexerError(e))?)
    }

    /// Parses the collection of tokens into a root naml node element.
    pub fn parse_tokens(tokens: &mut Vec<NamlToken>) -> Result<NamlNodeElement, NamlParserError> {
        let mut node_root = NamlNodeElement::default();

        while !tokens.is_empty() {
            let current = tokens.remove(0);
            if let BlockClose = current {
                // End of parsing for this node
                break;
            }

            match current {
                NamlToken::Text(var_name) => {
                    if tokens.is_empty() {
                        return Err(SyntaxError);
                    }
                    let operator = tokens.remove(0);
                    match operator {
                        NamlToken::BlockOpen => node_root
                            .insert(var_name, NamlValue::NamlNode(NamlParser::parse_tokens(tokens)?)),
                        NamlToken::AssignmentOperator => {
                            if tokens.is_empty() {
                                return Err(SyntaxError);
                            }
                            let value = tokens.remove(0);
                            node_root.insert(var_name, NamlParser::parse_value(value)?)
                        }
                        _ => {
                            return Err(SyntaxError);
                        }
                    }
                }
                _ => {
                    return Err(SyntaxError);
                }
            }
        }

        Ok(node_root)
    }

    /// Parses a passed token into a naml value interpreting it as a value.
    fn parse_value(token: NamlToken) -> Result<NamlValue, NamlParserError> {
        return match token {
            Text(content) => {
                return match content.deref() {
                    "y" => Ok(NamlValue::Boolean(true)),
                    "n" => Ok(NamlValue::Boolean(false)),
                    content => {
                        return if content.starts_with('"') && content.ends_with('"') {
                            Ok(NamlValue::String(content[1..content.len() - 1].to_string()))
                        } else if content.contains('.') {
                            Ok(NamlValue::Double(
                                content.parse().map_err(|_e| VariableParseError(format!("{} is not a f64", content)))?,
                            ))
                        } else {
                            Ok(NamlValue::Integer(
                                content.parse().map_err(|_e| VariableParseError(format!("{} is not an i64", content)))?,
                            ))
                        };
                    }
                };
            }
            _ => Err(UnknownVariable),
        };
    }
}

/// A collection of all errors that might be encountered while the parser attempts to parse the incoming token stream.
#[derive(Debug)]
pub enum NamlParserError {
    UnclosedBlock,
    BlocksClosedTooOften,
    SyntaxError,
    UnknownVariable,
    VariableParseError(String),
    LexerError(NamlLexerError)
}
