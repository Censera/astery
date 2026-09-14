use crate::Token;
use crate::compiler::{Program, Source};
use crate::error::Error;
use crate::span::{SourceSpan, Spanned};

/// Semantic analysis input retains the parser program and expanded token stream.
/// The parser program is the structural input. The token stream is retained for
/// source spans and macro-expanded source ownership. Semantic lowering has not
/// discarded parser information yet, so later semantic passes do not need to
/// reconstruct locations from source text.
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

    pub(crate) fn semantic_error(
        &self,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Error {
        Error::semantic(span, message)
    }

    pub(crate) fn semantic_error_at_token(
        &self,
        index: usize,
        message: impl Into<String>,
    ) -> Option<Error> {
        self.token_span(index)
            .map(|span| self.semantic_error(span, message))
    }

    pub(crate) fn semantic_error_for_range(
        &self,
        start: usize,
        end: usize,
        message: impl Into<String>,
    ) -> Option<Error> {
        self.span_for_range(start, end)
            .map(|span| self.semantic_error(span, message))
    }
}
