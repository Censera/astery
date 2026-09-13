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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Lex {
                line,
                column,
                message,
            } => write!(f, "E [{line}][{column}] {message}"),
            Self::Parse {
                line,
                column,
                message,
            } => write!(f, "E [{line}][{column}] {message}"),
            Self::WithSource { source, error } => match error.location() {
                Some((line, column)) => write!(
                    f,
                    "{} [{}][{}][{}] {}",
                    error.prefix(),
                    source,
                    line,
                    column,
                    error.message()
                ),
                None => write!(f, "{error}"),
            },
            Self::StageNotImplemented(stage) => {
                write!(f, "compiler stage is not implemented: {stage:?}")
            }
        }
    }
}

impl Error {
    fn prefix(&self) -> &str {
        match self {
            Self::Lex { .. } => "E",
            Self::Parse { .. } => "E",
            Self::WithSource { error, .. } => error.prefix(),
            _ => "E",
        }
    }

    fn location(&self) -> Option<(usize, usize)> {
        match self {
            Self::Lex { line, column, .. } | Self::Parse { line, column, .. } => {
                Some((*line, *column))
            }
            Self::WithSource { error, .. } => error.location(),
            _ => None,
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::Lex { message, .. } | Self::Parse { message, .. } => message,
            Self::WithSource { error, .. } => error.message(),
            _ => "",
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
