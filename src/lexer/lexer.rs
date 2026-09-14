#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

impl Token {
    fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }

    pub(crate) fn synthetic(kind: TokenKind, line: usize, column: usize) -> Self {
        Self::new(kind, line, column)
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn span(&self) -> crate::span::SourceSpan {
        crate::span::SourceSpan::point(crate::span::SourceLocation::new(self.line, self.column))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Character(char),
    Label(String),
    Mod,
    Use,
    Let,
    Const,
    Fn,
    Return,
    If,
    Elif,
    Else,
    Then,
    Break,
    Continue,
    Loop,
    While,
    Match,
    For,
    In,
    Enum,
    Struct,
    Into,
    Pub,
    Pri,
    Type,
    Embed,
    Macro,
    True,
    False,
    None,
    At,
    Arrow,
    And,
    Or,
    Xor,
    Not,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    Equal,
    NotEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    EqualSign,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Exclamation,
    Question,
    Caret,
    IncrementPlaceholder,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, crate::Error> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(ch) = chars.next() {
        let token_line = line;
        let token_column = column;
        column += ch.len_utf8();

        if ch == '\n' {
            line += 1;
            column = 1;
            continue;
        }

        if ch.is_whitespace() {
            continue;
        }

        let token = match ch {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '?' => TokenKind::Question,
            '^' => TokenKind::Caret,
            '@' => TokenKind::At,
            '+' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::AddAssign
                } else if chars.peek() == Some(&'+') {
                    chars.next();
                    column += 1;
                    TokenKind::Increment
                } else {
                    TokenKind::Add
                }
            }
            '-' => {
                if chars.peek() == Some(&'>') {
                    chars.next();
                    column += 1;
                    TokenKind::Arrow
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::SubAssign
                } else if chars.peek() == Some(&'-') {
                    chars.next();
                    column += 1;
                    TokenKind::Decrement
                } else {
                    TokenKind::Subtract
                }
            }
            '*' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::MulAssign
                } else {
                    TokenKind::Multiply
                }
            }
            '/' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::DivAssign
                } else {
                    TokenKind::Divide
                }
            }
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::Equal
                } else {
                    TokenKind::EqualSign
                }
            }
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::NotEqual
                } else {
                    TokenKind::Exclamation
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::GreaterEqual
                } else if chars.peek() == Some(&'>') {
                    chars.next();
                    column += 1;
                    TokenKind::ShiftRight
                } else {
                    TokenKind::Greater
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    column += 1;
                    TokenKind::LessEqual
                } else if chars.peek() == Some(&'<') {
                    chars.next();
                    column += 1;
                    TokenKind::ShiftLeft
                } else {
                    TokenKind::Less
                }
            }
            '&' => TokenKind::BitAnd,
            '|' => TokenKind::BitOr,
            '~' => TokenKind::BitNot,
            '"' => {
                let mut value = String::new();
                while let Some(next) = chars.next() {
                    column += next.len_utf8();
                    if next == '"' {
                        break;
                    }
                    value.push(next);
                }
                TokenKind::String(value)
            }
            '\'' => {
                let value = chars.next().ok_or_else(|| crate::Error::Lex {
                    line,
                    column,
                    message: "unterminated character literal".into(),
                })?;
                column += value.len_utf8();
                if chars.next() != Some('\'') {
                    return Err(crate::Error::Lex {
                        line,
                        column,
                        message: "unterminated character literal".into(),
                    });
                }
                column += 1;
                TokenKind::Character(value)
            }
            ch if ch.is_ascii_digit() => {
                let mut value = ch.to_string();
                let mut is_float = false;
                while let Some(next) = chars.peek().copied() {
                    if next.is_ascii_digit() {
                        value.push(next);
                        chars.next();
                        column += 1;
                    } else if next == '.' && !is_float {
                        is_float = true;
                        value.push(next);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                if is_float {
                    TokenKind::Float(value)
                } else {
                    TokenKind::Integer(value)
                }
            }
            ch if ch.is_ascii_alphabetic() || ch == '_' => {
                let mut value = ch.to_string();
                while let Some(next) = chars.peek().copied() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        value.push(next);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                match value.as_str() {
                    "mod" => TokenKind::Mod,
                    "use" => TokenKind::Use,
                    "let" => TokenKind::Let,
                    "const" => TokenKind::Const,
                    "fn" => TokenKind::Fn,
                    "return" => TokenKind::Return,
                    "if" => TokenKind::If,
                    "elif" => TokenKind::Elif,
                    "else" => TokenKind::Else,
                    "then" => TokenKind::Then,
                    "break" => TokenKind::Break,
                    "continue" => TokenKind::Continue,
                    "loop" => TokenKind::Loop,
                    "while" => TokenKind::While,
                    "match" => TokenKind::Match,
                    "for" => TokenKind::For,
                    "in" => TokenKind::In,
                    "enum" => TokenKind::Enum,
                    "struct" => TokenKind::Struct,
                    "into" => TokenKind::Into,
                    "pub" => TokenKind::Pub,
                    "pri" => TokenKind::Pri,
                    "type" => TokenKind::Type,
                    "embed" => TokenKind::Embed,
                    "macro" => TokenKind::Macro,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "None" => TokenKind::None,
                    "and" => TokenKind::And,
                    "or" => TokenKind::Or,
                    "xor" => TokenKind::Xor,
                    "not" => TokenKind::Not,
                    _ => {
                        if value.starts_with('_') {
                            TokenKind::Identifier(value)
                        } else {
                            TokenKind::Identifier(value)
                        }
                    }
                }
            }
            _ => {
                return Err(crate::Error::Lex {
                    line: token_line,
                    column: token_column,
                    message: format!("unexpected character `{ch}`"),
                })
            }
        };

        tokens.push(Token::new(token, token_line, token_column));
    }

    Ok(tokens)
}
