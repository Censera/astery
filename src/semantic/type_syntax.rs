use crate::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Parsed type syntax retained for semantic lowering.
pub enum TypeSyntax {
    Void,
    Bool,
    Char,
    String,
    Integer {
        signed: bool,
        bits: u16,
    },
    Float {
        bits: u16,
    },
    User(String),
    Generic {
        name: String,
        arguments: Vec<TypeSyntax>,
    },
    Parameter(String),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    pub name: String,
    pub parameters: Vec<String>,
    pub target: TypeSyntax,
}

pub fn parse(tokens: &[TokenKind]) -> Result<TypeSyntax, String> {
    parse_with_parameters(tokens, &[])
}

pub fn parse_with_parameters(
    tokens: &[TokenKind],
    parameters: &[String],
) -> Result<TypeSyntax, String> {
    if tokens.is_empty() {
        return Ok(TypeSyntax::Void);
    }
    let mut parser = Parser {
        tokens,
        parameters,
        position: 0,
    };
    let ty = parser.parse_type()?;
    if parser.position != tokens.len() {
        return Err("unexpected token after type".into());
    }
    Ok(ty)
}

pub fn parse_alias(
    name: impl Into<String>,
    parameters: Vec<String>,
    tokens: &[TokenKind],
) -> Result<TypeAlias, String> {
    let target = parse_with_parameters(tokens, &parameters)?;
    Ok(TypeAlias {
        name: name.into(),
        parameters,
        target,
    })
}

struct Parser<'a> {
    tokens: &'a [TokenKind],
    parameters: &'a [String],
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
        if self.parameters.iter().any(|parameter| parameter == &name) {
            return Ok(TypeSyntax::Parameter(name));
        }

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
            _ => {
                if self.take(TokenKind::Tetraops) {
                    self.parse_generic_application(name)?
                } else {
                    TypeSyntax::User(name)
                }
            }
        };
        Ok(ty)
    }

    fn parse_generic_application(&mut self, name: String) -> Result<TypeSyntax, String> {
        let mut arguments = Vec::new();
        if self.take(TokenKind::Greater) {
            return Err("generic type requires at least one argument".into());
        }
        loop {
            arguments.push(self.parse_type()?);
            if self.take(TokenKind::Comma) {
                continue;
            }
            self.expect(TokenKind::Greater, "expected `>` in generic type")?;
            break;
        }
        Ok(TypeSyntax::Generic { name, arguments })
    }

    fn parse_union(&mut self) -> Result<TypeSyntax, String> {
        if !self.take(TokenKind::Tetraops) {
            return Err("expected `::<` in union type".into());
        }
        let mut types = Vec::new();
        if self.take(TokenKind::Greater) {
            return Err("union type requires at least one member".into());
        }
        loop {
            types.push(self.parse_type()?);
            if self.take(TokenKind::Comma) {
                continue;
            }
            self.expect(TokenKind::Greater, "expected `>` in union type")?;
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

#[cfg(test)]
mod tests {
    use super::{TypeSyntax, parse, parse_alias};
    use crate::{TokenKind, lexer::tokenize};

    fn kinds(source: &str) -> Vec<TokenKind> {
        tokenize(source)
            .unwrap()
            .into_iter()
            .map(|token| token.kind().clone())
            .collect()
    }

    #[test]
    fn parses_parameterized_generic_union_aliases() {
        let alias = parse_alias(
            "Result",
            vec!["T".into(), "E".into()],
            &kinds("Union::<T, E>"),
        )
        .unwrap();
        assert_eq!(alias.name, "Result");
        assert_eq!(alias.parameters, vec!["T", "E"]);
        assert_eq!(
            alias.target,
            TypeSyntax::Union(vec![
                TypeSyntax::Parameter("T".into()),
                TypeSyntax::Parameter("E".into()),
            ])
        );
    }

    #[test]
    fn parses_concrete_generic_type_applications() {
        let ty = parse(&kinds("Result::<i64, string>")).unwrap();
        assert_eq!(
            ty,
            TypeSyntax::Generic {
                name: "Result".into(),
                arguments: vec![
                    TypeSyntax::Integer {
                        signed: true,
                        bits: 64,
                    },
                    TypeSyntax::String,
                ],
            }
        );
    }

    #[test]
    fn parses_parameterized_union_directly() {
        let ty = parse(&kinds("Union::<i64, string>")).unwrap();
        assert_eq!(
            ty,
            TypeSyntax::Union(vec![
                TypeSyntax::Integer {
                    signed: true,
                    bits: 64,
                },
                TypeSyntax::String,
            ])
        );
    }

    #[test]
    fn rejects_empty_generic_arguments() {
        let error = parse(&kinds("Result::<>")).unwrap_err();
        assert!(error.contains("generic type requires at least one argument"));
    }
}
