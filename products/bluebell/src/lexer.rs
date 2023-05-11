use regex::Regex;
use std::convert::From;
use std::iter::Peekable;
use std::str::CharIndices;
use std::string::String;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

const KEYWORD_FORALL: &'static str = "forall";
const KEYWORD_BUILTIN: &'static str = "builtin";
const KEYWORD_LIBRARY: &'static str = "library";
const KEYWORD_IMPORT: &'static str = "import";
const KEYWORD_LET: &'static str = "let";
const KEYWORD_IN: &'static str = "in";
const KEYWORD_MATCH: &'static str = "match";
const KEYWORD_WITH: &'static str = "with";
const KEYWORD_END: &'static str = "end";
const KEYWORD_FUN: &'static str = "fun";
const KEYWORD_TFUN: &'static str = "tfun";
const KEYWORD_CONTRACT: &'static str = "contract";
const KEYWORD_TRANSITION: &'static str = "transition";
const KEYWORD_SEND: &'static str = "send";
const KEYWORD_FIELD: &'static str = "field";
const KEYWORD_ACCEPT: &'static str = "accept";
const KEYWORD_EXISTS: &'static str = "exists";
const KEYWORD_DELETE: &'static str = "delete";
const KEYWORD_THROW: &'static str = "throw";
const KEYWORD_MAP: &'static str = "Map";
const KEYWORD_SCILLA_VERSION: &'static str = "scilla_version";
const KEYWORD_TYPE: &'static str = "type";
const KEYWORD_OF: &'static str = "of";
const KEYWORD_AS: &'static str = "as";
const KEYWORD_PROCEDURE: &'static str = "procedure";
const KEYWORD_EMP: &'static str = "Emp";
const KEYWORD_EVENT: &'static str = "event";
const KEYWORD_EVENT_TYPE: &'static str = "Event";
const KEYWORD_BYSTR: &'static str = "ByStr";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<S> {
    Plus,
    Asterisk,
    Semicolon,
    Colon,
    Dot,
    Pipe,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    DoubleArrow,
    Arrow,
    Equals,
    Ampersand,
    LeftArrow,
    ColonEquals,

    At,
    Minus,
    Underscore,
    Forall,
    Builtin,
    Library,
    Import,
    Let,
    In,
    Match,
    With,
    End,
    Fun,
    Tfun,
    Contract,
    Transition,
    Send,
    Field,
    Accept,
    Exists,
    Delete,
    Throw,
    Map,
    ScillaVersion,
    Type,
    Of,
    As,
    Procedure,
    Emp,
    Event,
    EventType,
    ByStr,
    ByStrWithSize(S),
    Comment(S),
    Number(S),
    HexNumber(S),
    Identifier(S),
    TemplateIdentifier(S),
    CustomIdentifier(S),
    SpecialIdentifier(S),
    TypeName(S),
    StringLiteral(S),
    Whitespace,

    Unknown,
}

impl<S: ToString> From<Token<S>> for String {
    fn from(token: Token<S>) -> Self {
        match token {
            Token::ByStrWithSize(value) => format!("{}", value.to_string()),
            Token::Comment(value) => format!("{}", value.to_string()),
            Token::Number(value) => format!("{}", value.to_string()),
            Token::HexNumber(value) => format!("{}", value.to_string()),
            Token::Identifier(value) => format!("{}", value.to_string()),
            Token::TemplateIdentifier(value) => format!("{}", value.to_string()),
            Token::CustomIdentifier(value) => format!("{}", value.to_string()),
            Token::SpecialIdentifier(value) => format!("{}", value.to_string()),
            Token::TypeName(value) => format!("{}", value.to_string()),
            Token::StringLiteral(value) => format!("{}", value.to_string()),
            _ => match token {
                Token::Plus => "+",
                Token::Asterisk => "*",
                Token::Semicolon => ";",
                Token::Colon => ":",
                Token::Dot => ".",
                Token::Pipe => "|",
                Token::OpenBracket => "[",
                Token::CloseBracket => "]",
                Token::OpenParen => "(",
                Token::CloseParen => ")",
                Token::OpenBrace => "{",
                Token::CloseBrace => "}",
                Token::Comma => ",",
                Token::DoubleArrow => "=>",
                Token::Arrow => "->",
                Token::Equals => "=",
                Token::Ampersand => "&",
                Token::LeftArrow => "<-",
                Token::ColonEquals => ":=",
                Token::At => "@",
                Token::Minus => "-",
                Token::Underscore => "_",
                Token::Forall => KEYWORD_FORALL,
                Token::Builtin => KEYWORD_BUILTIN,
                Token::Library => KEYWORD_LIBRARY,
                Token::Import => KEYWORD_IMPORT,
                Token::Let => KEYWORD_LET,
                Token::In => KEYWORD_IN,
                Token::Match => KEYWORD_MATCH,
                Token::With => KEYWORD_WITH,
                Token::End => KEYWORD_END,
                Token::Fun => KEYWORD_FUN,
                Token::Tfun => KEYWORD_TFUN,
                Token::Contract => KEYWORD_CONTRACT,
                Token::Transition => KEYWORD_TRANSITION,
                Token::Send => KEYWORD_SEND,
                Token::Field => KEYWORD_FIELD,
                Token::Accept => KEYWORD_ACCEPT,
                Token::Exists => KEYWORD_EXISTS,
                Token::Delete => KEYWORD_DELETE,
                Token::Throw => KEYWORD_THROW,
                Token::Map => KEYWORD_MAP,
                Token::ScillaVersion => KEYWORD_SCILLA_VERSION,
                Token::Type => KEYWORD_TYPE,
                Token::Of => KEYWORD_OF,
                Token::As => KEYWORD_AS,
                Token::Procedure => KEYWORD_PROCEDURE,
                Token::Emp => KEYWORD_EMP,
                Token::Event => KEYWORD_EVENT,
                Token::EventType => KEYWORD_EVENT_TYPE,
                Token::ByStr => KEYWORD_BYSTR,

                Token::Whitespace => " ",
                _ => "?", // Token::Unknown made as a wild card to avoid compiler complaining.
            }
            .to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    // Not possible
}

/// Provides the ability to tokenize a source file.
pub struct Lexer<'input> {
    /// An iterator of the source file's characters, along with their indices.
    chars: Peekable<CharIndices<'input>>,
    /// A reference to the source file being tokenized.
    document: &'input str,
    /// The current line number being tokenized.
    line: usize,
    /// The current character number within the current line being tokenized.
    character: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            document: input,
            line: 1,
            character: 1,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<&'input str>, usize, ParseError>;

    // <(usize, Token, usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch)) = self.chars.next() {
            let (token, end): (Token<&'input str>, usize) = {
                let look_ahead = self.chars.peek().map(|(_, next_ch)| *next_ch);
                self.character += ch.len_utf8();

                let next_is_alpha_num_under = look_ahead
                    .map(|c| c.is_alphanumeric() || c == '_')
                    .unwrap_or(false);
                let next_is_numeric = look_ahead.map(|c| c.is_numeric()).unwrap_or(false);

                // Handle more complex tokens, whitespace, and comments
                if ch.is_whitespace() {
                    if ch == '\n' {
                        self.character = 0;
                        self.line += 1;
                    }
                    continue;
                } else if ch == '=' && look_ahead == Some('>') {
                    self.chars.next();
                    (Token::DoubleArrow, start + 2 * ch.len_utf8())
                } else if ch == '-' && look_ahead == Some('>') {
                    self.chars.next();
                    (Token::Arrow, start + 2 * ch.len_utf8())
                } else if ch == '-' && !next_is_numeric {
                    (Token::Minus, start + ch.len_utf8())
                } else if ch == '<' && look_ahead == Some('-') {
                    self.chars.next();
                    (Token::LeftArrow, start + 2 * ch.len_utf8())
                } else if ch == ':' && look_ahead == Some('=') {
                    self.chars.next();
                    (Token::ColonEquals, start + 2 * ch.len_utf8())
                } else if ch == '_' && !next_is_alpha_num_under {
                    (Token::Underscore, start + ch.len_utf8())
                } else if ch == '(' && look_ahead == Some('*') {
                    // Consume comment

                    self.chars.next(); // Consume '*'
                    let mut comment = String::new();

                    while let Some((_, ch)) = self.chars.next() {
                        if ch == '*' && self.chars.peek().map(|(_, next_ch)| *next_ch) == Some(')')
                        {
                            self.chars.next();
                            break;
                        } else {
                            comment.push(ch);
                        }
                    }

                    continue;
                    // TODO: Hack to avoid emitting comment. However, ideally these should be part of the AST or at least the token stream
                    // let len = comment.len();
                    // let end = start + len + 2 + 1; // +2: `*)`, +1: move to char beyond last
                    // let s = &self.document[start + 2..end - 1]; // +2: skip `(*`
                    // (Token::Comment(s), end)
                } else {
                    let (token, end): (Token<&'input str>, usize) = match ch {
                        '+' => (Token::Plus, start + ch.len_utf8()),
                        '*' => (Token::Asterisk, start + ch.len_utf8()),
                        ';' => (Token::Semicolon, start + ch.len_utf8()),
                        ':' => (Token::Colon, start + ch.len_utf8()),
                        '.' => (Token::Dot, start + ch.len_utf8()),
                        '|' => (Token::Pipe, start + ch.len_utf8()),
                        '[' => (Token::OpenBracket, start + ch.len_utf8()),
                        ']' => (Token::CloseBracket, start + ch.len_utf8()),
                        '(' => (Token::OpenParen, start + ch.len_utf8()),
                        ')' => (Token::CloseParen, start + ch.len_utf8()),
                        '{' => (Token::OpenBrace, start + ch.len_utf8()),
                        '}' => (Token::CloseBrace, start + ch.len_utf8()),
                        ',' => (Token::Comma, start + ch.len_utf8()),
                        '&' => (Token::Ampersand, start + ch.len_utf8()),
                        '@' => (Token::At, start + ch.len_utf8()),
                        '=' => (Token::Equals, start + ch.len_utf8()),
                        _ => {
                            let token_str: &str = &self.document[start..];
                            let mut index = 0;
                            let token_str_chars = token_str.chars();
                            for (i, c) in token_str_chars.enumerate() {
                                if !c.is_alphanumeric() && c != '_' {
                                    index = i;
                                    break;
                                }
                            }
                            let keyword_token: &str = if index > 0 {
                                &token_str[..index]
                            } else {
                                token_str
                            };

                            let (token, end): (Token<&'input str>, usize) = match keyword_token {
                                KEYWORD_FORALL => {
                                    self.chars.nth(KEYWORD_FORALL.len() - 2);
                                    (Token::Forall, start + KEYWORD_FORALL.len())
                                }
                                KEYWORD_BUILTIN => {
                                    self.chars.nth(KEYWORD_BUILTIN.len() - 2);
                                    (Token::Builtin, start + KEYWORD_BUILTIN.len())
                                }
                                KEYWORD_LIBRARY => {
                                    self.chars.nth(KEYWORD_LIBRARY.len() - 2);
                                    (Token::Library, start + KEYWORD_LIBRARY.len())
                                }
                                KEYWORD_IMPORT => {
                                    self.chars.nth(KEYWORD_IMPORT.len() - 2);
                                    (Token::Import, start + KEYWORD_IMPORT.len())
                                }
                                KEYWORD_LET => {
                                    self.chars.nth(KEYWORD_LET.len() - 2);
                                    (Token::Let, start + KEYWORD_LET.len())
                                }
                                KEYWORD_IN => {
                                    self.chars.nth(KEYWORD_IN.len() - 2);
                                    (Token::In, start + KEYWORD_IN.len())
                                }
                                KEYWORD_MATCH => {
                                    self.chars.nth(KEYWORD_MATCH.len() - 2);
                                    (Token::Match, start + KEYWORD_MATCH.len())
                                }
                                KEYWORD_WITH => {
                                    self.chars.nth(KEYWORD_WITH.len() - 2);
                                    (Token::With, start + KEYWORD_WITH.len())
                                }
                                KEYWORD_END => {
                                    self.chars.nth(KEYWORD_END.len() - 2);
                                    (Token::End, start + KEYWORD_END.len())
                                }
                                KEYWORD_FUN => {
                                    self.chars.nth(KEYWORD_FUN.len() - 2);
                                    (Token::Fun, start + KEYWORD_FUN.len())
                                }
                                KEYWORD_TFUN => {
                                    self.chars.nth(KEYWORD_TFUN.len() - 2);
                                    (Token::Tfun, start + KEYWORD_TFUN.len())
                                }
                                KEYWORD_CONTRACT => {
                                    self.chars.nth(KEYWORD_CONTRACT.len() - 2);
                                    (Token::Contract, start + KEYWORD_CONTRACT.len())
                                }
                                KEYWORD_TRANSITION => {
                                    self.chars.nth(KEYWORD_TRANSITION.len() - 2);
                                    (Token::Transition, start + KEYWORD_TRANSITION.len())
                                }
                                KEYWORD_SEND => {
                                    self.chars.nth(KEYWORD_SEND.len() - 2);
                                    (Token::Send, start + KEYWORD_SEND.len())
                                }
                                KEYWORD_FIELD => {
                                    self.chars.nth(KEYWORD_FIELD.len() - 2);
                                    (Token::Field, start + KEYWORD_FIELD.len())
                                }
                                KEYWORD_ACCEPT => {
                                    self.chars.nth(KEYWORD_ACCEPT.len() - 2);
                                    (Token::Accept, start + KEYWORD_ACCEPT.len())
                                }
                                KEYWORD_EXISTS => {
                                    self.chars.nth(KEYWORD_EXISTS.len() - 2);
                                    (Token::Exists, start + KEYWORD_EXISTS.len())
                                }
                                KEYWORD_DELETE => {
                                    self.chars.nth(KEYWORD_DELETE.len() - 2);
                                    (Token::Delete, start + KEYWORD_DELETE.len())
                                }
                                KEYWORD_THROW => {
                                    self.chars.nth(KEYWORD_THROW.len() - 2);
                                    (Token::Throw, start + KEYWORD_THROW.len())
                                }
                                KEYWORD_MAP => {
                                    self.chars.nth(KEYWORD_MAP.len() - 2);
                                    (Token::Map, start + KEYWORD_MAP.len())
                                }
                                KEYWORD_SCILLA_VERSION => {
                                    self.chars.nth(KEYWORD_SCILLA_VERSION.len() - 2);
                                    (Token::ScillaVersion, start + KEYWORD_SCILLA_VERSION.len())
                                }
                                KEYWORD_TYPE => {
                                    self.chars.nth(KEYWORD_TYPE.len() - 2);
                                    (Token::Type, start + KEYWORD_TYPE.len())
                                }
                                KEYWORD_OF => {
                                    self.chars.nth(KEYWORD_OF.len() - 2);
                                    (Token::Of, start + KEYWORD_OF.len())
                                }
                                KEYWORD_AS => {
                                    self.chars.nth(KEYWORD_AS.len() - 2);
                                    (Token::As, start + KEYWORD_AS.len())
                                }
                                KEYWORD_PROCEDURE => {
                                    self.chars.nth(KEYWORD_PROCEDURE.len() - 2);
                                    (Token::Procedure, start + KEYWORD_PROCEDURE.len())
                                }
                                KEYWORD_EMP => {
                                    self.chars.nth(KEYWORD_EMP.len() - 2);
                                    (Token::Emp, start + KEYWORD_EMP.len())
                                }
                                KEYWORD_EVENT => {
                                    self.chars.nth(KEYWORD_EVENT.len() - 2);
                                    (Token::Event, start + KEYWORD_EVENT.len())
                                }
                                KEYWORD_EVENT_TYPE => {
                                    self.chars.nth(KEYWORD_EVENT_TYPE.len() - 2);
                                    (Token::EventType, start + KEYWORD_EVENT_TYPE.len())
                                }
                                _ => {
                                    // Handle other cases here
                                    let bystr_with_size = Regex::new(r"^ByStr[0-9]+").unwrap();

                                    let signed_integer = Regex::new(r"^[+-]?[0-9]+").unwrap();
                                    let hex_number =
                                        Regex::new(r"^0(x|X)([a-fA-F0-9][a-fA-F0-9])*").unwrap();
                                    let string_literal = Regex::new(r#"^"(?:\\.|[^"])*""#).unwrap();
                                    let regular_id = Regex::new(r"^[a-z][a-zA-Z0-9_]*").unwrap();
                                    let template_type_id =
                                        Regex::new(r"^['][A-Z][a-zA-Z0-9_]*").unwrap();
                                    let custom_type_id =
                                        Regex::new(r"^[A-Z][a-zA-Z0-9_]*").unwrap();
                                    let special_id = Regex::new(r"^[_][a-zA-Z0-9_]*").unwrap();

                                    if let Some(mat) = bystr_with_size.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2); // -2, because we already consumed the first char
                                        }

                                        (Token::ByStrWithSize(s), end)
                                    } else if token_str.starts_with(KEYWORD_BYSTR) {
                                        self.chars.nth(KEYWORD_BYSTR.len() - 2);
                                        (Token::ByStr, start + KEYWORD_BYSTR.len())
                                    } else if let Some(mat) = hex_number.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2); // -2, because we already consumed the first char
                                        }

                                        (Token::HexNumber(s), end)
                                    } else if let Some(mat) = signed_integer.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2); // -2, because we already consumed the first char
                                        }
                                        (Token::Number(s), end)
                                    } else if let Some(mat) = string_literal.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2); // -2, because we already consumed the first char
                                        }

                                        (Token::StringLiteral(s), end)
                                    } else if let Some(mat) = regular_id.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2); // -2, because we already consumed the first char
                                        }

                                        (Token::Identifier(s), end)
                                    } else if let Some(mat) = template_type_id.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2); // -2, because we already consumed the first char
                                        }

                                        (Token::TemplateIdentifier(s), end)
                                    } else if let Some(mat) = custom_type_id.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2);
                                        }
                                        (Token::CustomIdentifier(s), end)
                                    } else if let Some(mat) = special_id.find(token_str) {
                                        let end = start + mat.end();
                                        let s = &self.document[start..end];
                                        if mat.end() > 1 {
                                            self.chars.nth(end - start - 2); // -2, because we already consumed the first char
                                        }

                                        (Token::SpecialIdentifier(s), end)
                                    } else {
                                        (Token::Unknown, start)
                                    }
                                }
                            };

                            (token, end)
                        }
                    };
                    (token, end)
                }
            };

            return Some(Ok((start, token, end)));
        }

        None
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test {
        ($src:expr, $($span:expr => $token:expr,)*) => {{
            let lexed_tokens: Vec<_> = Lexer::new($src.into()).collect();
            let expected_tokens : Vec<Result<(usize, Token<&str>, usize), ParseError>>= vec![$({
                let start : usize = $span.find("~").unwrap() as usize;
                let end : usize = $span.rfind("~").unwrap() as usize;
                Ok((start, $token, end))
            }),*];

            assert_eq!(lexed_tokens, expected_tokens);
        }};
    }

    // TODO: Integrate comments into the AST
    #[test]
    fn doc_comment() {
        test! {
            "       (* hello Scilla *)",
            "       ~~~~~~~~~~~~~~~~~~" => Token::Comment(" hello Scilla "),
        };
        test! {
            "       (***** hello *****)",
            "       ~~~~~~~~~~~~~~~~~~~" => Token::Comment("**** hello ****"),
        };
        test! {
            "       (* *** hello ** **)",
            "       ~~~~~~~~~~~~~~~~~~~" => Token::Comment(" *** hello ** *"),
        };
        test! {
            "       (*(*(* hello *(*(*)",
            "       ~~~~~~~~~~~~~~~~~~~" => Token::Comment("(*(* hello *(*("),
        };
    }

    // TODO: Add support for
    // (* Fish (* Soup *) *)
    // (* Fish (* Soup  *)
}
*/
