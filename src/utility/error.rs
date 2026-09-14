use std::fmt;

use crate::span::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Source text could not be converted into tokens.
    Lexer,
    /// Tokens do not form valid Astery syntax or structure.
    Parser,
    /// Parsed constructs are structurally valid but violate Astery meaning or type rules.
    Semantic,
    /// A valid semantic program could not be lowered to the target backend.
    Backend,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidOptions(String),
    UnsupportedTarget(String),
    Lex {
        line: usize,
        column: usize,
        message: String,
    },
    Parse {
        line: usize,
        column: usize,
        message: String,
    },
    Semantic {
        span: SourceSpan,
        message: String,
    },
    Backend(String),
    WithSource {
        source: String,
        error: Box<Error>,
    },
    StageNotImplemented(Stage),
}

impl Error {
    pub fn stage(&self) -> Option<Stage> {
        match self {
            Self::Io(_) | Self::InvalidOptions(_) | Self::UnsupportedTarget(_) => None,
            Self::Lex { .. } => Some(Stage::Lexer),
            Self::Parse { .. } => Some(Stage::Parser),
            Self::Semantic { .. } => Some(Stage::Semantic),
            Self::Backend(_) => Some(Stage::Backend),
            Self::WithSource { error, .. } => error.stage(),
            Self::StageNotImplemented(stage) => Some(*stage),
        }
    }

    pub fn with_source(self, source: impl Into<String>) -> Self {
        Self::WithSource {
            source: source.into(),
            error: Box::new(self),
        }
    }

    pub fn semantic(span: SourceSpan, message: impl Into<String>) -> Self {
        Self::Semantic {
            span,
            message: message.into(),
        }
    }

    fn diagnostic(&self) -> Option<Diagnostic<'_>> {
        match self {
            Self::Lex {
                line,
                column,
                message,
            } => Some(Diagnostic::Point {
                kind: "E",
                line: *line,
                column: *column,
                message,
            }),
            Self::Parse {
                line,
                column,
                message,
            } => Some(Diagnostic::Point {
                kind: "E",
                line: *line,
                column: *column,
                message,
            }),
            Self::Semantic { span, message } => Some(Diagnostic::Span {
                kind: "E",
                span: *span,
                message,
            }),
            Self::WithSource { error, .. } => error.diagnostic(),
            _ => None,
        }
    }
}

enum Diagnostic<'a> {
    Point {
        kind: &'static str,
        line: usize,
        column: usize,
        message: &'a str,
    },
    Span {
        kind: &'static str,
        span: SourceSpan,
        message: &'a str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::InvalidOptions(message) => write!(f, "invalid compiler options: {message}"),
            Self::UnsupportedTarget(target) => write!(f, "unsupported target: {target}"),
            Self::Backend(message) => write!(f, "backend error: {message}"),
            Self::WithSource { source, error } => {
                match error.diagnostic() {
                    Some(Diagnostic::Point {
                        kind,
                        line,
                        column,
                        message,
                    }) => write!(f, "{kind} [{source}][{line}][{column}] {message}"),
                    Some(Diagnostic::Span {
                        kind,
                        span,
                        message,
                    }) => write!(
                        f,
                        "{kind} [{source}][{}:{}-{}:{}] {message}",
                        span.start.line,
                        span.start.column,
                        span.end.line,
                        span.end.column,
                    ),
                    None => write!(f, "{error}"),
                }
            }
            Self::Lex {
                line,
                column,
                message,
            }
            | Self::Parse {
                line,
                column,
                message,
            } => write!(f, "E [{line}][{column}] {message}"),
            Self::Semantic { span, message } => write!(
                f,
                "E [{}:{}-{}:{}] {message}",
                span.start.line,
                span.start.column,
                span.end.line,
                span.end.column,
            ),
            Self::StageNotImplemented(stage) => {
                write!(f, "compiler stage is not implemented: {stage:?}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
