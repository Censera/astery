use crate::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSyntax {
    Void,
    Bool,
    Char,
    String,
    SignedInteger(u16),
    UnsignedInteger(u16),
    Float(u16),
    Identifier(String),
    Pointer(Box<TypeSyntax>),
    OptionalPointer(Box<TypeSyntax>),
    Array {
        element: Box<TypeSyntax>,
        length: Option<String>,
    },
    Vector {
        element: Box<TypeSyntax>,
        length: Option<String>,
    },
    Tuple(Vec<TypeSyntax>),
    Union(Vec<TypeSyntax>),
}

pub fn parse(tokens: &[TokenKind]) -> Result<TypeSyntax, String> {
    super::type_syntax_impl::parse(tokens)
}
