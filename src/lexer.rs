use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

use logos::{Logos, SpannedIter};

use crate::Integer;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}

#[derive(Debug, Default, Error, Clone, PartialEq, Eq)]
pub enum LexicalError {
    #[error("Integer parsing error: {0}")]
    InvalidInteger(#[from] ParseIntError),
    #[error("Float parsing error: {0}")]
    InvalidFloat(#[from] ParseFloatError),
    #[default]
    #[error("Encountered invalid token")]
    InvalidToken,
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexicalError)]
#[logos(skip r"[\s\t\r\n\f]+")] // Whitespace
#[logos(skip r"//[^\n\r]*[\n\r]*")] // Inline comments
#[logos(skip r"/\*([^*/]|\*[^/]|/[^*])*\*/")] // Multiline comments
pub enum Token {
    #[token("let")]
    Let,

    #[token("in")]
    In,

    #[token("null")]
    Null,

    #[token("=")]
    Equals,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token(".")]
    Chain,

    #[token("..")]
    Spread,

    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLiteral(String),

    #[regex(r"-[0-9]+(_[0-9]+)*", |lex| lex.slice().replace("_", "").parse::<i64>().map(Integer::from))]
    #[regex(r"[0-9]+(_[0-9]+)*", |lex| lex.slice().replace("_", "").parse::<u64>().map(Integer::from))]
    Integer(Integer),

    #[regex(r"-?[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>())]
    Float(f64),

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Boolean(bool),

    #[regex(r"\$[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    InputName(String),

    #[regex(r"'[^']*'|[^\s.=0-9\[\]{}-][^\s.=]*", |lex| lex.slice().trim_matches('\'').to_string())]
    Key(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let => write!(f, "let"),
            Self::In => write!(f, "in"),
            Self::Null => write!(f, "null"),
            Self::Equals => write!(f, "="),
            Self::OpenBrace => write!(f, "{{"),
            Self::CloseBrace => write!(f, "}}"),
            Self::OpenBracket => write!(f, "["),
            Self::CloseBracket => write!(f, "]"),
            Self::Chain => write!(f, "."),
            Self::Spread => write!(f, ".."),
            Self::StringLiteral(lit) => lit.fmt(f),
            Self::Integer(int) => int.fmt(f),
            Self::Float(float) => float.fmt(f),
            Self::Boolean(bool) => bool.fmt(f),
            Self::InputName(name) => name.fmt(f),
            Self::Key(key) => key.fmt(f),
        }
    }
}
