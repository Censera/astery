use crate::compiler::{Program, Source};
use crate::error::Error;
use crate::lexer::Token;
use crate::parser::{FunctionDeclaration, ImportItem, Visibility};
use crate::span::{SourceSpan, Spanned};

mod type_syntax;

use type_syntax::{TypeSyntax, parse as parse_type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SemanticVisibility {
    Public,
    Private,
}

impl From<Visibility> for SemanticVisibility {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Public => Self::Public,
            Visibility::Private => Self::Private,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompilerAttribute {
    Striped,
    Lossely,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct SemanticFunctionAttributes {
    pub(crate) compiler: Vec<CompilerAttribute>,
}

impl SemanticFunctionAttributes {
    pub(crate) fn from_flags(flags: &[String]) -> Self {
        let compiler = flags
            .iter()
            .map(|flag| match flag.as_str() {
                "striped" => CompilerAttribute::Striped,
                "lossely" => CompilerAttribute::Lossely,
                _ => CompilerAttribute::Unknown(flag.clone()),
            })
            .collect();
        Self { compiler }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SemanticFunctionSignature {
    pub(crate) parameters: Vec<TypeSyntax>,
    pub(crate) return_type: TypeSyntax,
}

impl SemanticFunctionSignature {
    pub(crate) fn from_function(function: &FunctionDeclaration) -> Result<Self, String> {
        let parameters = function
            .parameters
            .iter()
            .map(|parameter| parse_type(&parameter.type_tokens))
            .collect::<Result<Vec<_>, _>>()?;
        let return_type = parse_type(&function.return_type)?;
        Ok(Self {
            parameters,
            return_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SemanticOverloadSet {
    pub(crate) name: String,
    pub(crate) signatures: Vec<SemanticFunctionSignature>,
}

impl SemanticOverloadSet {
    pub(crate) fn from_functions(
        functions: &[FunctionDeclaration],
    ) -> Result<Vec<Self>, String> {
        let mut overloads = Vec::new();
        for function in functions {
            let signature = SemanticFunctionSignature::from_function(function)?;
            if let Some(set) = overloads.iter_mut().find(|set: &&mut Self| set.name == function.name) {
                set.signatures.push(signature);
            } else {
                overloads.push(Self {
                    name: function.name.clone(),
                    signatures: vec![signature],
                });
            }
        }
        Ok(overloads)
    }
}

/// Semantic analysis owns the source, the complete parser program, and the
/// macro-expanded token stream that produced that program.
///
/// The parser program is the structural input to semantic analysis. The token
/// stream is stored as `Spanned<Token>` so semantic lowering can attach exact
/// source locations to names, types, expressions, and diagnostics without
/// reparsing source text or guessing locations from identifier names.
///
/// Source text remains owned as well so diagnostics can identify the source
/// file and future reporting can inspect the original source when necessary.
///
/// When semantic lowering introduces its own representation, parser-only
/// syntax details may be discarded once no diagnostic or backend requirement
/// depends on them. Semantic names, resolved declarations, resolved types,
/// required expression structure, and their source locations must remain.
#[derive(Debug)]
pub(crate) struct SemanticProgram {
    pub(crate) source: Source,
    pub(crate) module: SemanticModule,
    pub(crate) program: Program,
    pub(crate) tokens: Vec<Spanned<Token>>,
}

/// The root module and imports belonging to one parsed source unit.
#[derive(Debug)]
pub(crate) struct SemanticModule {
    pub(crate) name: Option<String>,
    pub(crate) imports: Vec<SemanticImport>,
}

#[derive(Debug)]
pub(crate) struct SemanticImport {
    pub(crate) module: Option<String>,
    pub(crate) items: Vec<SemanticImportItem>,
}

#[derive(Debug)]
pub(crate) struct SemanticImportItem {
    pub(crate) name: String,
    pub(crate) items: Vec<SemanticImportItem>,
}

impl SemanticModule {
    pub(crate) fn from_program(program: &Program) -> Self {
        Self {
            name: program.module.as_ref().map(|module| module.name.clone()),
            imports: program
                .imports
                .iter()
                .map(|import| SemanticImport {
                    module: import.module.clone(),
                    items: import
                        .items
                        .iter()
                        .map(SemanticImportItem::from_parser)
                        .collect(),
                })
                .collect(),
        }
    }
}

impl SemanticImportItem {
    fn from_parser(item: &ImportItem) -> Self {
        Self {
            name: item.name.clone(),
            items: item.items.iter().map(Self::from_parser).collect(),
        }
    }
}

impl SemanticProgram {
    pub(crate) fn new(source: Source, program: Program, tokens: Vec<Token>) -> Self {
        let tokens = tokens
            .into_iter()
            .map(|token| {
                let span = token.span();
                Spanned::new(token, span)
            })
            .collect();
        let module = SemanticModule::from_program(&program);
        Self {
            source,
            module,
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

    pub(crate) fn tokens(&self) -> &[Spanned<Token>] {
        &self.tokens
    }

    pub(crate) fn token(&self, index: usize) -> Option<&Spanned<Token>> {
        self.tokens.get(index)
    }

    pub(crate) fn token_span(&self, index: usize) -> Option<SourceSpan> {
        self.tokens.get(index).map(Spanned::span)
    }

    pub(crate) fn span_for_range(&self, start: usize, end: usize) -> Option<SourceSpan> {
        if start >= end {
            return None;
        }

        let first = self.tokens.get(start)?;
        let last = self.tokens.get(end - 1)?;
        Some(SourceSpan::covering(first.span.start, last.span.end))
    }

    pub(crate) fn semantic_error(&self, span: SourceSpan, message: impl Into<String>) -> Error {
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

#[cfg(test)]
mod tests {
    use super::{
        CompilerAttribute, SemanticFunctionAttributes, SemanticFunctionSignature,
        SemanticOverloadSet, SemanticVisibility,
    };
    use crate::lexer::tokenize;
    use crate::parser::Visibility;
    use crate::parser::parse_functions;

    #[test]
    fn lowers_visibility() {
        assert_eq!(
            SemanticVisibility::from(Visibility::Public),
            SemanticVisibility::Public
        );
        assert_eq!(
            SemanticVisibility::from(Visibility::Private),
            SemanticVisibility::Private
        );
    }

    #[test]
    fn preserves_known_and_unknown_function_flags() {
        let flags = vec!["striped".into(), "lossely".into(), "custom".into()];
        let attributes = SemanticFunctionAttributes::from_flags(&flags);
        assert_eq!(
            attributes.compiler,
            vec![
                CompilerAttribute::Striped,
                CompilerAttribute::Lossely,
                CompilerAttribute::Unknown("custom".into()),
            ]
        );
    }

    #[test]
    fn builds_function_signature() {
        let tokens = tokenize("fn i32 add(a i32, b i32) {}").unwrap();
        let function = &parse_functions(&tokens).unwrap()[0];
        let signature = SemanticFunctionSignature::from_function(function).unwrap();
        assert_eq!(signature.parameters.len(), 2);
        assert_eq!(signature.return_type, super::TypeSyntax::Integer { signed: true, bits: 32 });
    }

    #[test]
    fn groups_functions_into_overload_sets() {
        let tokens = tokenize(
            "fn i32 add(a i32) {} fn f64 add(a f64) {} fn i32 sub(a i32) {}",
        )
        .unwrap();
        let functions = parse_functions(&tokens).unwrap();
        let overloads = SemanticOverloadSet::from_functions(&functions).unwrap();
        assert_eq!(overloads.len(), 2);
        assert_eq!(overloads[0].name, "add");
        assert_eq!(overloads[0].signatures.len(), 2);
        assert_eq!(overloads[1].name, "sub");
        assert_eq!(overloads[1].signatures.len(), 1);
    }
}
