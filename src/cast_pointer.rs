use crate::{Error, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExpression {
    pub value: Vec<TokenKind>,
    pub target_type: Vec<TokenKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointerType {
    pub optional: bool,
    pub type_tokens: Vec<TokenKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressOfExpression {
    pub value: Vec<TokenKind>,
}

pub fn parse_cast(tokens: &[Token]) -> Result<CastExpression, Error> {
    let mut parser = Parser { tokens, position: 0 };
    let value = parser.collect_until(TokenKind::Arrow)?;
    parser.expect(TokenKind::Arrow)?;
    let target_type = parser.collect_remaining();
    if target_type.is_empty() {
        return Err(parser.error("expected cast target type"));
    }
    Ok(CastExpression { value, target_type })
}

pub fn parse_pointer_type(tokens: &[Token]) -> Result<PointerType, Error> {
    let mut parser = Parser { tokens, position: 0 };
    let optional = if parser.peek() == Some(&TokenKind::Question) {
        parser.advance();
        true
    } else {
        false
    };
    parser.expect(TokenKind::Caret)?;
    parser.expect(TokenKind::OpenBracket)?;
    if parser.peek() == Some(&TokenKind::CloseBracket) {
        return Err(parser.error("expected pointer type"));
    }
    let type_tokens = parser.collect_until(TokenKind::CloseBracket)?;
    if type_tokens.is_empty() {
        return Err(parser.error("expected pointer type"));
    }
    parser.expect(TokenKind::CloseBracket)?;
    parser.finish()?;
    Ok(PointerType {
        optional,
        type_tokens,
    })
}

pub fn parse_address_of(tokens: &[Token]) -> Result<AddressOfExpression, Error> {
    let mut parser = Parser { tokens, position: 0 };
    parser.expect(TokenKind::Ampersand)?;
    let value = parser.collect_remaining();
    if value.is_empty() {
        return Err(parser.error("expected address-of value"));
    }
    Ok(AddressOfExpression { value })
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position).map(Token::kind)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), Error> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error("unexpected token"))
        }
    }

    fn collect_until(&mut self, stop: TokenKind) -> Result<Vec<TokenKind>, Error> {
        let start = self.position;
        while let Some(kind) = self.peek() {
            if kind == &stop {
                break;
            }
            self.advance();
        }
        if self.peek().is_none() {
            return Err(self.error("unexpected end of input"));
        }
        if self.position == start {
            return Err(self.error("expected expression"));
        }
        Ok(self.tokens[start..self.position]
            .iter()
            .map(Token::kind)
            .cloned()
            .collect())
    }

    fn collect_remaining(&mut self) -> Vec<TokenKind> {
        let start = self.position;
        while self.peek().is_some() {
            self.advance();
        }
        self.tokens[start..self.position]
            .iter()
            .map(Token::kind)
            .cloned()
            .collect()
    }

    fn finish(&self) -> Result<(), Error> {
        if self.peek().is_some() {
            return Err(self.error("unexpected token after pointer type"));
        }
        Ok(())
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
    use super::{AddressOfExpression, CastExpression, PointerType, parse_address_of, parse_cast, parse_pointer_type};
    use crate::{TokenKind, lexer::tokenize};

    #[test]
    fn parses_cast() {
        let tokens = tokenize("a -> i8").unwrap();
        assert_eq!(
            parse_cast(&tokens).unwrap(),
            CastExpression {
                value: vec![TokenKind::Identifier("a".into())],
                target_type: vec![TokenKind::Identifier("i8".into())],
            }
        );
    }

    #[test]
    fn parses_pointer_type() {
        let tokens = tokenize("^[i32]").unwrap();
        assert_eq!(
            parse_pointer_type(&tokens).unwrap(),
            PointerType {
                optional: false,
                type_tokens: vec![TokenKind::Identifier("i32".into())],
            }
        );
    }

    #[test]
    fn parses_optional_pointer_type() {
        let tokens = tokenize("?^[string]").unwrap();
        assert_eq!(
            parse_pointer_type(&tokens).unwrap(),
            PointerType {
                optional: true,
                type_tokens: vec![TokenKind::Identifier("string".into())],
            }
        );
    }

    #[test]
    fn parses_address_of() {
        let tokens = tokenize("&value").unwrap();
        assert_eq!(
            parse_address_of(&tokens).unwrap(),
            AddressOfExpression {
                value: vec![TokenKind::Identifier("value".into())],
            }
        );
    }

    #[test]
    fn rejects_empty_cast_target() {
        let tokens = tokenize("a ->").unwrap();
        let error = parse_cast(&tokens).unwrap_err();
        assert!(error.to_string().contains("expected cast target type"));
    }

    #[test]
    fn rejects_invalid_pointer_type() {
        let tokens = tokenize("^[ ]").unwrap();
        let error = parse_pointer_type(&tokens).unwrap_err();
        assert!(error.to_string().contains("expected pointer type"));
    }
}