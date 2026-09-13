use crate::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl SourceSpan {
    pub fn point(line: usize, column: usize) -> Self {
        let position = SourcePosition { line, column };
        Self {
            start: position,
            end: position,
        }
    }

    pub fn from_tokens(tokens: &[Token]) -> Option<Self> {
        let first = tokens.first()?;
        let last = tokens.last()?;
        Some(Self {
            start: SourcePosition {
                line: first.line(),
                column: first.column(),
            },
            end: SourcePosition {
                line: last.line(),
                column: last.column(),
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Located<T> {
    pub value: T,
    pub span: SourceSpan,
}

impl<T> Located<T> {
    pub fn new(value: T, span: SourceSpan) -> Self {
        Self { value, span }
    }
}
