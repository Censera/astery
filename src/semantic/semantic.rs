use crate::Token;
use crate::compiler::{Program, Source};
use crate::error::Error;
use crate::span::{SourceSpan, Spanned};

/// Semantic analysis owns the source, the complete parser program, and the
/// macro-expanded token stream that produced that program.
///
/// The parser program is the structural input to semantic analysis. The token
/// stream remains owned by the semantic stage because parser nodes currently
/// do not carry their own spans, and later diagnostics must not reconstruct
/// locations by reparsing source text.
///
/// Source text remains owned as well so diagnostics can identify the source
/// file and future reporting can inspect the original source when necessary.
///
/// When semantic lowering introduces its own representation, parser-only
/// syntax details may be discarded once no diagnostic or backend requirement
/// depends on them. Semantic names, resolved declarations, resolved types,
/// required expression structure, and source locations must remain. The
/// expanded token stream is retained until equivalent source-location data is
/// represented directly by the semantic model.
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
