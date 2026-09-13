use crate::span::{Located, SourceSpan};
use crate::type_syntax::TypeSyntax;
use crate::{Token, TokenKind};

pub type SemanticName = Located<String>;
pub type SemanticType = Located<TypeSyntax>;

pub fn name_input(token: &Token) -> SemanticName {
    let name = match token.kind() {
        TokenKind::Identifier(name) => name.clone(),
        kind => format!("{kind:?}"),
    };
    Located::new(name, SourceSpan::point(token.line(), token.column()))
}

pub fn type_input(tokens: &[Token]) -> Result<SemanticType, String> {
    let kinds = tokens.iter().map(Token::kind).cloned().collect::<Vec<_>>();
    let syntax = TypeSyntax::parse(&kinds)?;
    let span = SourceSpan::from_tokens(tokens).ok_or_else(|| "expected type".to_owned())?;
    Ok(Located::new(syntax, span))
}
