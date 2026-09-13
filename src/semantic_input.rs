use super::span::{Located, SourceSpan};
use super::type_syntax::{self, TypeSyntax};
use crate::Token;

pub type SemanticType = Located<TypeSyntax>;

pub fn type_input(tokens: &[Token]) -> Result<SemanticType, String> {
    let kinds = tokens.iter().map(Token::kind).cloned().collect::<Vec<_>>();
    let syntax = type_syntax::parse(&kinds)?;
    let span = SourceSpan::from_tokens(tokens).ok_or_else(|| "expected type".to_owned())?;
    Ok(Located::new(syntax, span))
}
