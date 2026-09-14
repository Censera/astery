use crate::Token;
use crate::compiler::{Program, Source};
use crate::span::SourceSpan;

#[derive(Debug)]
pub(crate) struct SemanticProgram {
    pub(crate) source: Source,
    pub(crate) program: Program,
    pub(crate) tokens: Vec<Token>,
}

impl SemanticProgram {
    pub(crate) fn new(source: Source, program: Program, tokens: Vec<Token>) -> Self {
        Self {
            source,
            program,
            tokens,
        }
    }

    pub(crate) fn source_name(&self) -> &str {
        self.source.name()
    }

    pub(crate) fn source_text(&self) -> &str {
        self.source.text()
    }

    pub(crate) fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub(crate) fn token_span(&self, index: usize) -> Option<SourceSpan> {
        self.tokens.get(index).map(Token::span)
    }

    pub(crate) fn source_span(&self, line: usize, column: usize) -> SourceSpan {
        let location = crate::span::SourceLocation::new(line, column);
        SourceSpan::point(location)
    }
}
