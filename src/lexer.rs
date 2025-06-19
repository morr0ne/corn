use std::{
    borrow::Cow,
    fmt,
    num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

use logos::{Logos, SpannedIter};

use crate::Integer;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, Token<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexicalError>;

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
pub enum Token<'input> {
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

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Boolean(bool),

    #[regex(r"-[0-9]+(_[0-9]+)*", |lex| lex.slice().replace("_", "").parse::<i64>().map(Integer::from))]
    #[regex(r"[0-9]+(_[0-9]+)*", |lex| lex.slice().replace("_", "").parse::<u64>().map(Integer::from))]
    // FIXME: this is a mess sob
    #[regex(r"0x[0-9a-fA-F][_0-9a-fA-F]*", |lex| i64::from_str_radix(&lex.slice().trim_start_matches("0x").replace("_", ""), 16).map(Integer::from))]
    Integer(Integer),

    #[regex(r"-?[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>())]
    Float(f64),

    #[regex(r"\$[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().trim_start_matches('$'))]
    InputName(&'input str),

    #[token("\"", parse_literal)]
    Literal(Vec<StringPart<'input>>),

    #[regex(r#"'(?:[^']|(\\'))*'|[^\s.=0-9\[\]{}"'][^\s.=\[\]{}"']*"#, |lex| lex.slice().trim_matches('\'').replace("\\'", "'"))]
    Key(String),
}

#[derive(Logos, Debug, PartialEq, Clone)]
enum StringContext<'input> {
    #[token("\"")]
    Quote,
    #[regex(r#"[^\"$\\{]+"#)]
    Content,

    #[token("\\n")]
    NewlineEscape,
    #[token("\\r")]
    CarriageReturnEscape,
    #[token("\\t")]
    TabEscape,
    #[token("\\\\")]
    BackslashEscape,
    #[token("\\\"")]
    QuoteEscape,
    #[token("\\$")]
    DollarEscape,
    #[token("\\{")]
    OpenBraceEscape,
    #[token("\\}")]
    CloseBraceEscape,
    #[regex(r"\\u[0-9a-fA-F]{4}")]
    UnicodeEscape,

    #[regex(r"\$\{[a-zA-Z_][a-zA-Z0-9_]*\}", |lex| lex.slice())]
    Interpolation(&'input str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart<'input> {
    Literal(Cow<'input, str>),
    Input(&'input str),
}

fn parse_literal<'input>(
    lex: &mut logos::Lexer<'input, Token<'input>>,
) -> Option<Vec<StringPart<'input>>> {
    let mut string_lex = lex.clone().morph::<StringContext>();

    let mut parts = Vec::new();
    let mut current_literal = String::new();

    while let Some(Ok(token)) = string_lex.next() {
        match token {
            StringContext::Quote => break,
            StringContext::Content => current_literal.push_str(string_lex.slice()),
            StringContext::Interpolation(input) => {
                if !current_literal.is_empty() {
                    parts.push(StringPart::Literal(Cow::Owned(std::mem::take(
                        &mut current_literal,
                    ))));
                }

                parts.push(StringPart::Input(
                    input.trim_start_matches("${").trim_end_matches('}'),
                ))
            }
            StringContext::NewlineEscape => {
                current_literal.push('\n');
            }
            StringContext::CarriageReturnEscape => {
                current_literal.push('\r');
            }
            StringContext::TabEscape => {
                current_literal.push('\t');
            }
            StringContext::BackslashEscape => {
                current_literal.push('\\');
            }
            StringContext::QuoteEscape => {
                current_literal.push('"');
            }
            StringContext::DollarEscape => {
                current_literal.push('$');
            }
            StringContext::OpenBraceEscape => {
                current_literal.push('{');
            }
            StringContext::CloseBraceEscape => {
                current_literal.push('}');
            }
            StringContext::UnicodeEscape => {
                let slice = string_lex.slice();
                let hex_part = &slice[2..]; // Skip "\\u"

                if let Ok(code) = u32::from_str_radix(hex_part, 16) {
                    if let Some(unicode_char) = char::from_u32(code) {
                        current_literal.push(unicode_char);
                        continue;
                    }
                }

                current_literal.push('\u{FFFD}');
            }
        }
    }

    if !current_literal.is_empty() {
        parts.push(StringPart::Literal(Cow::Owned(std::mem::take(
            &mut current_literal,
        ))));
    }

    *lex = string_lex.morph();

    Some(parts)
}

impl fmt::Display for Token<'_> {
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
            Self::Literal(parts) => {
                for part in parts {
                    match part {
                        StringPart::Literal(lit) => write!(f, "{lit}")?,
                        StringPart::Input(input) => write!(f, "{input}")?,
                    }
                }

                Ok(())
            }
            Self::Integer(int) => int.fmt(f),
            Self::Float(float) => float.fmt(f),
            Self::Boolean(bool) => bool.fmt(f),
            Self::InputName(name) => name.fmt(f),
            Self::Key(key) => key.fmt(f),
        }
    }
}
