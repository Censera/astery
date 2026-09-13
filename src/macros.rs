use crate::{Error, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDeclaration {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroCall {
    pub name: String,
    pub arguments: Vec<Vec<Token>>,
}

pub fn parse_macros(tokens: &[Token]) -> Result<Vec<MacroDeclaration>, Error> {
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    let mut macros = Vec::new();
    while parser.peek() == Some(&TokenKind::Macro) {
        macros.push(parser.parse_macro()?);
    }
    parser.finish()?;
    Ok(macros)
}

pub fn parse_macro_call(tokens: &[Token]) -> Result<MacroCall, Error> {
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    let name = match parser.advance() {
        Some(TokenKind::Identifier(name)) => name,
        _ => return Err(parser.error("expected macro name")),
    };
    parser.expect(TokenKind::Exclamation)?;
    parser.expect(TokenKind::OpenParen)?;
    let arguments = parser.parse_arguments()?;
    parser.expect(TokenKind::CloseParen)?;
    parser.finish()?;
    Ok(MacroCall { name, arguments })
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    fn parse_macro(&mut self) -> Result<MacroDeclaration, Error> {
        self.expect(TokenKind::Macro)?;
        let name = match self.advance() {
            Some(TokenKind::Identifier(name)) => name,
            _ => return Err(self.error("expected macro name")),
        };
        self.expect(TokenKind::OpenParen)?;
        let parameters = self.parse_parameters()?;
        self.expect(TokenKind::CloseParen)?;
        self.expect(TokenKind::OpenBrace)?;
        let body = self.collect_balanced_body()?;
        Ok(MacroDeclaration {
            name,
            parameters,
            body,
        })
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, Error> {
        let mut parameters = Vec::new();
        if self.peek() == Some(&TokenKind::CloseParen) {
            return Ok(parameters);
        }
        loop {
            let name = match self.advance() {
                Some(TokenKind::Identifier(name)) => name,
                _ => return Err(self.error("expected macro parameter")),
            };
            parameters.push(name);
            if self.peek() != Some(&TokenKind::Comma) {
                break;
            }
            self.advance();
            if self.peek() == Some(&TokenKind::CloseParen) {
                return Err(self.error("expected macro parameter"));
            }
        }
        Ok(parameters)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Vec<Token>>, Error> {
        let mut arguments = Vec::new();
        if self.peek() == Some(&TokenKind::CloseParen) {
            return Ok(arguments);
        }
        loop {
            let argument = self.collect_until_argument_separator()?;
            arguments.push(argument);
            match self.peek() {
                Some(TokenKind::Comma) => {
                    self.advance();
                    if self.peek() == Some(&TokenKind::CloseParen) {
                        return Err(self.error("expected macro argument"));
                    }
                }
                Some(TokenKind::CloseParen) => break,
                _ => return Err(self.error("expected `,` or `)` in macro call")),
            }
        }
        Ok(arguments)
    }

    fn collect_until_argument_separator(&mut self) -> Result<Vec<Token>, Error> {
        let start = self.position;
        let mut depth = 0usize;
        while let Some(kind) = self.peek() {
            if depth == 0 && matches!(kind, TokenKind::Comma | TokenKind::CloseParen) {
                break;
            }
            match kind {
                TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => depth += 1,
                TokenKind::CloseParen | TokenKind::CloseBracket | TokenKind::CloseBrace => {
                    if depth == 0 {
                        return Err(self.error("unexpected closing delimiter in macro argument"));
                    }
                    depth -= 1;
                }
                _ => {}
            }
            self.advance();
        }
        if self.position == start {
            return Err(self.error("expected macro argument"));
        }
        if depth != 0 {
            return Err(self.error("unterminated delimiter in macro argument"));
        }
        Ok(self.tokens[start..self.position].to_vec())
    }

    fn collect_balanced_body(&mut self) -> Result<Vec<Token>, Error> {
        let start = self.position;
        let mut depth = 1usize;
        while let Some(kind) = self.peek() {
            match kind {
                TokenKind::OpenBrace => depth += 1,
                TokenKind::CloseBrace => {
                    depth -= 1;
                    if depth == 0 {
                        let body = self.tokens[start..self.position].to_vec();
                        self.advance();
                        return Ok(body);
                    }
                }
                _ => {}
            }
            self.advance();
        }
        Err(self.error("unterminated macro body"))
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), Error> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(self.error("unexpected token"))
        }
    }

    fn finish(&self) -> Result<(), Error> {
        if self.peek().is_some() {
            Err(self.error("unexpected token after macro"))
        } else {
            Ok(())
        }
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position).map(Token::kind)
    }

    fn advance(&mut self) -> Option<TokenKind> {
        let kind = self.tokens.get(self.position)?.kind().clone();
        self.position += 1;
        Some(kind)
    }

    fn error(&self, message: &str) -> Error {
        let token = self.tokens.get(self.position);
        let (line, column) = token
            .map(|token| (token.line(), token.column()))
            .unwrap_or_else(|| {
                self.tokens
                    .last()
                    .map(|token| (token.line(), token.column() + 1))
                    .unwrap_or((1, 1))
            });
        Error::Parse {
            line,
            column,
            message: message.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MacroCall, MacroDeclaration, parse_macro_call, parse_macros};
    use crate::{TokenKind, lexer::tokenize};

    #[test]
    fn parses_macros() {
        let tokens = tokenize("macro this() { \"this\" } macro square(x) { x * x }").unwrap();
        let macros = parse_macros(&tokens).unwrap();
        assert_eq!(macros[0].name, "this");
        assert!(matches!(macros[0].body[0].kind(), TokenKind::String(value) if value == "this"));
        assert_eq!(macros[1].name, "square");
        assert_eq!(macros[1].parameters, vec!["x"]);
        assert!(matches!(macros[1].body[0].kind(), TokenKind::Identifier(value) if value == "x"));
    }

    #[test]
    fn parses_macro_call() {
        let tokens = tokenize("square!(4)").unwrap();
        let call = parse_macro_call(&tokens).unwrap();
        assert_eq!(call.name, "square");
        assert_eq!(call.arguments.len(), 1);
        assert!(matches!(call.arguments[0][0].kind(), TokenKind::Integer(value) if value == "4"));
    }

    #[test]
    fn parses_nested_macro_arguments() {
        let tokens = tokenize("square!(1 + (2 * 3), [4, 5])").unwrap();
        let call = parse_macro_call(&tokens).unwrap();
        assert_eq!(call.name, "square");
        assert_eq!(call.arguments.len(), 2);
        assert!(matches!(call.arguments[0][0].kind(), TokenKind::Integer(value) if value == "1"));
        assert!(matches!(call.arguments[1][0].kind(), TokenKind::OpenBracket));
    }

    #[test]
    fn preserves_nested_macro_body_delimiters() {
        let tokens = tokenize("macro pair(a, b) { (a, b) }").unwrap();
        let macro_declaration = &parse_macros(&tokens).unwrap()[0];
        assert_eq!(macro_declaration.parameters, vec!["a", "b"]);
        assert!(matches!(macro_declaration.body[0].kind(), TokenKind::OpenParen));
        assert!(matches!(macro_declaration.body[1].kind(), TokenKind::Identifier(name) if name == "a"));
        assert!(matches!(macro_declaration.body[2].kind(), TokenKind::Comma));
        assert!(matches!(macro_declaration.body[3].kind(), TokenKind::Identifier(name) if name == "b"));
        assert!(matches!(macro_declaration.body[4].kind(), TokenKind::CloseParen));
    }

    #[test]
    fn rejects_empty_macro_argument() {
        let tokens = tokenize("square!()").unwrap();
        assert!(parse_macro_call(&tokens).unwrap().arguments.is_empty());
    }

    #[test]
    fn rejects_missing_macro_parameter() {
        let tokens = tokenize("macro square(,) { x }").unwrap();
        let error = parse_macros(&tokens).unwrap_err();
        assert!(error.to_string().contains("expected macro parameter"));
    }
}
