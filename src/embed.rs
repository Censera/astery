use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedBlock {
    pub language: String,
    pub body: String,
}

pub fn parse_embedded_blocks(source: &str) -> Result<Vec<EmbeddedBlock>, Error> {
    let mut parser = Parser::new(source);
    let mut blocks = Vec::new();

    parser.skip_whitespace_and_comments();
    while !parser.is_at_end() {
        parser.expect_keyword("embed")?;
        parser.skip_whitespace();
        let language = parser.parse_identifier()?;
        parser.skip_whitespace();
        let body = parser.parse_block()?;
        blocks.push(EmbeddedBlock { language, body });
        parser.skip_whitespace_and_comments();
    }

    Ok(blocks)
}

struct Parser<'a> {
    source: &'a str,
    bytes: &'a [u8],
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.bytes.len()
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.position + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.position += 1;
        if byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(byte)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.advance();
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();
            if self.peek() == Some(b'/') && self.peek_next() == Some(b'/') {
                self.advance();
                self.advance();
                while let Some(byte) = self.advance() {
                    if byte == b'\n' {
                        break;
                    }
                }
                continue;
            }
            if self.peek() == Some(b'/') && self.peek_next() == Some(b'*') {
                self.advance();
                self.advance();
                while !self.is_at_end() {
                    if self.peek() == Some(b'*') && self.peek_next() == Some(b'/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn expect_keyword(&mut self, expected: &str) -> Result<(), Error> {
        if self
            .bytes
            .get(self.position..self.position + expected.len())
            == Some(expected.as_bytes())
        {
            let after = self.position + expected.len();
            if after == self.bytes.len() || !is_identifier_continue(self.bytes[after]) {
                for _ in 0..expected.len() {
                    self.advance();
                }
                return Ok(());
            }
        }
        Err(self.error("expected `embed`"))
    }

    fn parse_identifier(&mut self) -> Result<String, Error> {
        let start = self.position;
        if !self.peek().is_some_and(is_identifier_start) {
            return Err(self.error("expected embed language"));
        }
        self.advance();
        while self.peek().is_some_and(is_identifier_continue) {
            self.advance();
        }
        Ok(self.source[start..self.position].to_owned())
    }

    fn parse_block(&mut self) -> Result<String, Error> {
        if self.peek() != Some(b'{') {
            return Err(self.error("expected `{` after embed language"));
        }
        self.advance();
        let body_start = self.position;
        let mut depth = 1usize;

        while let Some(byte) = self.peek() {
            match byte {
                b'"' | b'\'' => self.skip_quoted(byte)?,
                b'/' if self.peek_next() == Some(b'/') => self.skip_line_comment(),
                b'/' if self.peek_next() == Some(b'*') => self.skip_block_comment()?,
                b'{' => {
                    depth += 1;
                    self.advance();
                }
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        let body = self.source[body_start..self.position].to_owned();
                        self.advance();
                        return Ok(body);
                    }
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }

        Err(self.error("unterminated embedded block"))
    }

    fn skip_quoted(&mut self, quote: u8) -> Result<(), Error> {
        self.advance();
        while let Some(byte) = self.advance() {
            match byte {
                b'\\' => {
                    if self.advance().is_none() {
                        return Err(self.error("unterminated escape sequence in embedded block"));
                    }
                }
                byte if byte == quote => return Ok(()),
                _ => {}
            }
        }
        Err(self.error("unterminated quoted string in embedded block"))
    }

    fn skip_line_comment(&mut self) {
        self.advance();
        self.advance();
        while let Some(byte) = self.advance() {
            if byte == b'\n' {
                break;
            }
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), Error> {
        self.advance();
        self.advance();
        while !self.is_at_end() {
            if self.peek() == Some(b'*') && self.peek_next() == Some(b'/') {
                self.advance();
                self.advance();
                return Ok(());
            }
            self.advance();
        }
        Err(self.error("unterminated comment in embedded block"))
    }

    fn error(&self, message: &str) -> Error {
        Error::Parse {
            line: self.line,
            column: self.column,
            message: message.to_owned(),
        }
    }
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::{EmbeddedBlock, parse_embedded_blocks};

    #[test]
    fn parses_c_block() {
        assert_eq!(
            parse_embedded_blocks("embed C {\n    printf(\"Hello\\n\");\n}").unwrap(),
            vec![EmbeddedBlock {
                language: "C".into(),
                body: "\n    printf(\"Hello\\n\");\n".into(),
            }]
        );
    }

    #[test]
    fn preserves_nested_braces_and_comments() {
        let source = r#"embed C {
if (value) {
    printf("}");
    /* } */
}
}"#;
        let blocks = parse_embedded_blocks(source).unwrap();
        assert_eq!(blocks[0].language, "C");
        assert_eq!(
            blocks[0].body,
            "\nif (value) {\n    printf(\"}\");\n    /* } */\n}\n"
        );
    }

    #[test]
    fn parses_multiple_blocks() {
        let blocks = parse_embedded_blocks("embed C {}\n\nembed C { int x = 1; }").unwrap();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].body, "");
        assert_eq!(blocks[1].body, " int x = 1; ");
    }

    #[test]
    fn rejects_unterminated_block() {
        let error = parse_embedded_blocks("embed C { printf(\"hi\");").unwrap_err();
        assert!(error.to_string().contains("unterminated embedded block"));
    }

    #[test]
    fn rejects_missing_language() {
        let error = parse_embedded_blocks("embed { printf(\"hi\"); }").unwrap_err();
        assert!(error.to_string().contains("expected embed language"));
    }
}
