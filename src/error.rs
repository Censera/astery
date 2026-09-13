use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Lexer,
    Parser,
    Semantic,
    Backend,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
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
        line: usize,
        column: usize,
        message: String,
    },
    WithSource {
        source: String,
        error: Box<Error>,
    },
    StageNotImplemented(Stage),
}

impl Error {
    pub fn stage(&self) -> Option<Stage> {
        match self {
            Self::Io(_) => None,
            Self::Lex { .. } => Some(Stage::Lexer),
            Self::Parse { .. } => Some(Stage::Parser),
            Self::Semantic { .. } => Some(Stage::Semantic),
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

    fn diagnostic(&self) -> Option<(&str, usize, usize, &str)> {
        match self {
            Self::Lex {
                line,
                column,
                message,
            } => Some(("E", *line, *column, message)),
            Self::Parse {
                line,
                column,
                message,
            } => Some(("E", *line, *column, message)),
            Self::Semantic {
                line,
                column,
                message,
            } => Some(("E", *line, *column, message)),
            Self::WithSource { error, .. } => error.diagnostic(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::WithSource { source, error } => {
                if let Some((kind, line, column, message)) = error.diagnostic() {
                    write!(f, "{kind} [{source}][{line}][{column}] {message}")
                } else {
                    write!(f, "{error}")
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
            }
            | Self::Semantic {
                line,
                column,
                message,
            } => write!(f, "E [{line}][{column}] {message}"),
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
