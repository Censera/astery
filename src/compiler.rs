use crate::cast_pointer::{
    AddressOfExpression, CastExpression, PointerType, parse_address_of, parse_cast,
    parse_pointer_type,
};
use crate::embed::{EmbeddedBlock, parse_embedded_blocks};
use crate::error::{Error, Stage};
use crate::lexer::{Token, tokenize};
use crate::parser::{
    BindingDeclaration, EnumDeclaration, FunctionDeclaration, Import, IntoImplementation,
    ModuleDeclaration, StructDeclaration, parse_bindings, parse_enums, parse_functions,
    parse_imports, parse_intos, parse_module, parse_structs,
};
use crate::shortcuts::{MacroCall, MacroDeclaration, parse_macro_call, parse_macros};
use crate::user_type::{UserTypeDeclaration, parse_user_types};

#[path = "parser_contract.rs"]
mod parser_contract;
#[path = "type_syntax.rs"]
mod type_syntax;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    name: String,
    text: String,
}

impl Source {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
