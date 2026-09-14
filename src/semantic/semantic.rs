use crate::Token;
use crate::compiler::{Program, Source};
use crate::span::{SourceSpan, Spanned};

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

    pub(crate) fn token(&self, index: usize) -> Option<Spanned<&Token>> {
        self.tokens
            .get(index)
            .map(|token| Spanned::new(token, token.span()))
    }

    pub(crate) fn token_span(&self, index: usize) -> Option<SourceSpan> {
        self.tokens.get(index).map(Token::span)
    }

    pub(crate) fn span_for_range(&self, start: usize, end: usize) -> Option<SourceSpan> {
        if start >= end {
            return None;
        }

        let first = self.tokens.get(start)?;
        let last = self.tokens.get(end - 1)?;
        Some(SourceSpan::covering(first.span().start, last.span().end))
    }

    pub(crate) fn source_span(&self, line: usize, column: usize) -> SourceSpan {
        let location = crate::span::SourceLocation::new(line, column);
        SourceSpan::point(location)
    }
}
