use crate::parser::StructField;
use crate::{Error, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserTypeDeclaration {
    pub name: String,
    pub definition: UserTypeDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserTypeDefinition {
    Alias(Vec<TokenKind>),
    Struct(Vec<StructField>),
}

pub fn parse_user_types(tokens: &[Token]) -> Result<Vec<UserTypeDeclaration>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse()
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    fn parse(mut self) -> Result<Vec<UserTypeDeclaration>, Error> {
        let mut declarations = Vec::new();
        while self.peek() == Some(&TokenKind::Type) {
            declarations.push(self.parse_type()?);
        }
        if self.peek().is_some() {
            return Err(self.error("unexpected token after user-defined type"));
        }
        Ok(declarations)
    }

    fn parse_type(&mut self) -> Result<UserTypeDeclaration, Error> {
        self.expect(TokenKind::Type)?;
        let name = self.expect_identifier("expected user-defined type name")?;

        let definition = if self.peek() == Some(&TokenKind::Struct) {
            self.advance();
            UserTypeDefinition::Struct(self.parse_struct_fields()?)
        } else {
            UserTypeDefinition::Alias(self.parse_alias_tokens())
        };

        if matches!(definition, UserTypeDefinition::Alias(ref tokens) if tokens.is_empty()) {
            return Err(self.error("expected user-defined type definition"));
        }

        Ok(UserTypeDeclaration { name, definition })
    }

    fn parse_struct_fields(&mut self) -> Result<Vec<StructField>, Error> {
        self.expect(TokenKind::OpenBrace)?;
        let mut fields = Vec::new();
        while self.peek() != Some(&TokenKind::CloseBrace) {
            if self.peek().is_none() {
                return Err(self.error("unterminated user-defined struct"));
            }
            let visibility = match self.peek() {
                Some(TokenKind::Pub) => {
                    self.advance();
                    Some(crate::parser::Visibility::Public)
                }
                Some(TokenKind::Pri) => {
                    self.advance();
                    Some(crate::parser::Visibility::Private)
                }
                _ => None,
            };
            let name = self.expect_identifier("expected user-defined struct field name")?;
            let type_tokens = self.collect_until(|kind| {
                matches!(kind, TokenKind::Comma | TokenKind::CloseBrace)
            });
            if type_tokens.is_empty() {
                return Err(self.error("expected user-defined struct field type"));
            }
            fields.push(StructField {
                visibility,
                name,
                type_tokens,
            });
            if self.peek() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in user-defined struct"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(fields)
    }

    fn parse_alias_tokens(&mut self) -> Vec<TokenKind> {
        let start = self.position;
        let mut angle_depth = 0usize;
        while let Some(kind) = self.peek() {
            if angle_depth == 0 && matches!(kind, TokenKind::Type | TokenKind::Semicolon) {
                break;
            }
            match kind {
                TokenKind::Less | TokenKind::OpenAngle => angle_depth += 1,
                TokenKind::Greater | TokenKind::CloseAngle if angle_depth > 0 => angle_depth -= 1,
                _ => {}
            }
            self.advance();
        }
        self.tokens[start..self.position]
            .iter()
            .map(Token::kind)
            .cloned()
            .collect()
    }

    fn collect_until<F>(&mut self, stop: F) -> Vec<TokenKind>
    where
        F: Fn(&TokenKind) -> bool,
    {
        let start = self.position;
        while let Some(kind) = self.peek() {
            if stop(kind) {
                break;
            }
            self.advance();
        }
        self.tokens[start..self.position]
            .iter()
            .map(Token::kind)
            .cloned()
            .collect()
    }

    fn expect_identifier(&mut self, message: &str) -> Result<String, Error> {
        match self.peek() {
            Some(TokenKind::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(self.error(message)),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), Error> {
        if self.peek() == Some(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("expected token `{kind:?}`")))
        }
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position).map(Token::kind)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn error(&self, message: &str) -> Error {
        let token = self
            .tokens
            .get(self.position)
            .or_else(|| self.tokens.last());
        let (line, column) = token
            .map(|token| (token.line(), token.column()))
            .unwrap_or((1, 1));
        Error::Parse {
            line,
            column,
            message: message.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UserTypeDefinition, parse_user_types};
    use crate::{TokenKind, tokenize};

    #[test]
    fn parses_alias_user_type() {
        let tokens = tokenize("type Point i64").unwrap();
        let types = parse_user_types(&tokens).unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].name, "Point");
        assert_eq!(
            types[0].definition,
            UserTypeDefinition::Alias(vec![TokenKind::Identifier("i64".into())])
        );
    }

    #[test]
    fn parses_inline_struct_user_type() {
        let tokens = tokenize("type Node struct { pub value i64, next ^Node }").unwrap();
        let types = parse_user_types(&tokens).unwrap();
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].name, "Node");
        match &types[0].definition {
            UserTypeDefinition::Struct(fields) => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "value");
                assert_eq!(fields[1].name, "next");
            }
            definition => panic!("unexpected definition: {definition:?}"),
        }
    }

    #[test]
    fn parses_multiple_user_types() {
        let tokens = tokenize("type Point i64 type Value string").unwrap();
        let types = parse_user_types(&tokens).unwrap();
        assert_eq!(types.len(), 2);
        assert_eq!(types[0].name, "Point");
        assert_eq!(types[1].name, "Value");
    }

    #[test]
    fn rejects_missing_definition() {
        let tokens = tokenize("type Point").unwrap();
        let error = parse_user_types(&tokens).unwrap_err();
        assert!(error.to_string().contains("expected user-defined type definition"));
    }

    #[test]
    fn rejects_missing_struct_field_type() {
        let tokens = tokenize("type Node struct { value }").unwrap();
        let error = parse_user_types(&tokens).unwrap_err();
        assert!(error
            .to_string()
            .contains("expected user-defined struct field type"));
    }
}
