
use crate::lexer::naml_token::NamlToken;
use crate::lexer::naml_token::NamlToken::{AssignmentOperator, BlockClose, BlockOpen, Text};

use std::string::FromUtf8Error;

use crate::lexer::naml_lexer_text_index::NamlLexerTextTracker;


/// The naml lexer is capable of converting an incoming char stream into a stream of tokens.
pub struct NamlLexer;

impl NamlLexer {
    /// Tokenizes the parsed input text.
    pub fn tokenize(input: &Vec<u8>) -> Result<Vec<NamlToken>, NamlLexerError> {
        let mut tokens: Vec<NamlToken> = Vec::new();
        let buffer = String::from_utf8(input.clone()).map_err(|e| NamlLexerError::ReadError(e))?;

        let mut text_index = NamlLexerTextTracker::create();

        let line: Vec<char> = buffer.chars().into_iter().collect();
        for (i, char) in line.iter().enumerate() {
            let to_push_opt = match char {
                '{' => Some(BlockOpen),
                '}' => Some(BlockClose),
                '=' => Some(AssignmentOperator),
                ' ' | '\t' => None,
                '\n' => {
                    NamlLexer::try_to_push_text_token(&mut tokens, &mut text_index, &line);
                    None
                }
                _ => {
                    text_index.start_if_not_started(i);
                    text_index.bump_length();
                    None
                }
            };
            if let Some(to_push) = to_push_opt {
                NamlLexer::try_to_push_text_token(&mut tokens, &mut text_index, &line);
                tokens.push(to_push);
            }
        }

        NamlLexer::try_to_push_text_token(&mut tokens, &mut text_index, &line);
        Ok(tokens)
    }

    #[inline(always)]
    fn try_to_push_text_token(tokens: &mut Vec<NamlToken>, text_index: &mut NamlLexerTextTracker, line: &Vec<char>) {
        if text_index.is_tracking() {
            tokens.push(Text(line[text_index.start().unwrap()..text_index.end().unwrap()].iter().collect()));
            text_index.reset()
        }
    }
}

/// A collection of all errors that might be encountered while the lexer attempts to parse the incoming char stream.
#[derive(Debug)]
pub enum NamlLexerError {
    ReadError(FromUtf8Error),
}
