use crate::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSyntax {
    Void,
    Bool,
    Char,
    String,
    Integer { signed: bool, bits: u16 },
    Float { bits: u16 },
    User(String),
    Pointer {
        optional: bool,
        pointee: Box<TypeSyntax>,
    },
    Array {
        element: Box<TypeSyntax>,
        length: usize,
    },
    Vector {
        element: Box<TypeSyntax>,
        length: usize,
    },
    Tuple(Vec<TypeSyntax>),
    Union(Vec<TypeSyntax>),
}

pub fn parse(tokens: &[TokenKind]) -> Result<TypeSyntax, String> {
    if tokens.is_empty() {
        return Ok(TypeSyntax::Void);
    }
    let mut parser = Parser { tokens, position: 0 };
    let ty = parser.parse_type()?;
    if parser.position != tokens.len() {
        return Err("unexpected token after type".into());
    }
    Ok(ty)
}

struct Parser<'a> {
    tokens: &'a [TokenKind],
    position: usize,
}

impl<'a> Parser<'a> {
    fn parse_type(&mut self) -> Result<TypeSyntax, String> {
        let optional = self.take(TokenKind::Question);
        let pointer = self.take(TokenKind::Caret);
        if pointer {
            let pointee = self.parse_type()?;
            return Ok(TypeSyntax::Pointer {
                optional,
                pointee: Box::new(pointee),
            });
        }
        if optional {
            return Err("expected `^` after `?` in optional pointer type".into());
        }

        let mut ty = match self.advance() {
            Some(TokenKind::Identifier(name)) => self.parse_named_type(name)?,
            Some(TokenKind::OpenParen) => {
                let mut items = Vec::new();
                if !self.take(TokenKind::CloseParen) {
                    loop {
                        items.push(self.parse_type()?);
                        if self.take(TokenKind::Comma) {
                            continue;
                        }
                        self.expect(TokenKind::CloseParen, "expected `)` in tuple type")?;
                        break;
                    }
                }
                TypeSyntax::Tuple(items)
            }
            Some(other) => return Err(format!("expected type, found `{other:?}`")),
            None => return Err("expected type".into()),
        };

        loop {
            ty = match self.peek() {
                Some(TokenKind::OpenBracket) => {
                    self.position += 1;
                    let length = match self.advance() {
                        Some(TokenKind::Integer(value)) => value
                            .parse::<usize>()
                            .map_err(|_| "array length is out of range".to_owned())?,
                        _ => return Err("expected integer array length".into()),
                    };
                    self.expect(TokenKind::CloseBracket, "expected `]` in array type")?;
                    TypeSyntax::Array {
                        element: Box::new(ty),
                        length,
                    }
                }
                Some(TokenKind::OpenAngle) => {
                    self.position += 1;
                    let length = match self.advance() {
                        Some(TokenKind::Integer(value)) => value
                            .parse::<usize>()
                            .map_err(|_| "vector length is out of range".to_owned())?,
                        _ => return Err("expected integer vector length".into()),
                    };
                    self.expect(TokenKind::CloseAngle, "expected `>` in vector type")?;
                    TypeSyntax::Vector {
                        element: Box::new(ty),
                        length,
                    }
                }
                _ => break,
            };
        }

        Ok(ty)
    }

    fn parse_named_type(&mut self, name: String) -> Result<TypeSyntax, String> {
        let ty = match name.as_str() {
            "bool" => TypeSyntax::Bool,
            "char" => TypeSyntax::Char,
            "string" => TypeSyntax::String,
            "i8" => TypeSyntax::Integer {
                signed: true,
                bits: 8,
            },
            "i16" => TypeSyntax::Integer {
                signed: true,
                bits: 16,
            },
            "i32" => TypeSyntax::Integer {
                signed: true,
                bits: 32,
            },
            "i64" => TypeSyntax::Integer {
                signed: true,
                bits: 64,
            },
            "i128" => TypeSyntax::Integer {
                signed: true,
                bits: 128,
            },
            "u8" => TypeSyntax::Integer {
                signed: false,
                bits: 8,
            },
            "u16" => TypeSyntax::Integer {
                signed: false,
                bits: 16,
            },
            "u32" => TypeSyntax::Integer {
                signed: false,
                bits: 32,
            },
            "u64" => TypeSyntax::Integer {
                signed: false,
                bits: 64,
            },
            "u128" => TypeSyntax::Integer {
                signed: false,
                bits: 128,
            },
            "f32" => TypeSyntax::Float { bits: 32 },
            "f64" => TypeSyntax::Float { bits: 64 },
            "Union" => self.parse_union()?,
            _ => TypeSyntax::User(name),
        };
        Ok(ty)
    }

    fn parse_union(&mut self) -> Result<TypeSyntax, String> {
        self.expect(TokenKind::DoubleColon, "expected `::` in union type")?;
        self.expect(TokenKind::OpenAngle, "expected `<` in union type")?;
        let mut types = Vec::new();
        loop {
            types.push(self.parse_type()?);
            if self.take(TokenKind::Comma) {
                continue;
            }
            self.expect(TokenKind::CloseAngle, "expected `>` in union type")?;
            break;
        }
        Ok(TypeSyntax::Union(types))
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<TokenKind> {
        let token = self.tokens.get(self.position).cloned();
        if token.is_some() {
            self.position += 1;
        }
        token
    }

    fn take(&mut self, expected: TokenKind) -> bool {
        if self.peek() == Some(&expected) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: TokenKind, message: &str) -> Result<(), String> {
        if self.take(expected) {
            Ok(())
        } else {
            Err(message.into())
        }
    }
}
