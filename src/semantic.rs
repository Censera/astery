use crate::compiler::{Program, Source};
use crate::span::SourceSpan;

#[derive(Debug)]
pub(crate) struct SemanticProgram {
    pub(crate) source: Source,
    pub(crate) program: Program,
}

impl SemanticProgram {
    pub(crate) fn new(source: Source, program: Program) -> Self {
        Self { source, program }
    }

    pub(crate) fn source_name(&self) -> &str {
        self.source.name()
    }

    pub(crate) fn source_text(&self) -> &str {
        self.source.text()
    }

    pub(crate) fn source_span(&self, line: usize, column: usize) -> SourceSpan {
        let location = crate::span::SourceLocation::new(line, column);
        SourceSpan::new(location, location)
    }
}
