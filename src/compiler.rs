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
use crate::user_type::{UserTypeDeclaration, parse_user_types};

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

#[derive(Debug, Default)]
pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn tokenize(&self, source: &Source) -> Result<Vec<Token>, Error> {
        tokenize(source.text()).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_module(&self, source: &Source) -> Result<ModuleDeclaration, Error> {
        let tokens = self.tokenize(source)?;
        parse_module(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_imports(&self, source: &Source) -> Result<Vec<Import>, Error> {
        let tokens = self.tokenize(source)?;
        parse_imports(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_enums(&self, source: &Source) -> Result<Vec<EnumDeclaration>, Error> {
        let tokens = self.tokenize(source)?;
        parse_enums(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_structs(&self, source: &Source) -> Result<Vec<StructDeclaration>, Error> {
        let tokens = self.tokenize(source)?;
        parse_structs(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_intos(&self, source: &Source) -> Result<Vec<IntoImplementation>, Error> {
        let tokens = self.tokenize(source)?;
        parse_intos(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_user_types(&self, source: &Source) -> Result<Vec<UserTypeDeclaration>, Error> {
        let tokens = self.tokenize(source)?;
        parse_user_types(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_cast(&self, source: &Source) -> Result<CastExpression, Error> {
        let tokens = self.tokenize(source)?;
        parse_cast(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_pointer_type(&self, source: &Source) -> Result<PointerType, Error> {
        let tokens = self.tokenize(source)?;
        parse_pointer_type(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_address_of(&self, source: &Source) -> Result<AddressOfExpression, Error> {
        let tokens = self.tokenize(source)?;
        parse_address_of(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_embedded_blocks(&self, source: &Source) -> Result<Vec<EmbeddedBlock>, Error> {
        parse_embedded_blocks(source.text()).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_bindings(&self, source: &Source) -> Result<Vec<BindingDeclaration>, Error> {
        let tokens = self.tokenize(source)?;
        parse_bindings(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn parse_functions(&self, source: &Source) -> Result<Vec<FunctionDeclaration>, Error> {
        let tokens = self.tokenize(source)?;
        parse_functions(&tokens).map_err(|error| error.with_source(source.name()))
    }

    pub fn compile(&self, source: Source) -> Result<(), Error> {
        self.parse_imports(&source)?;
        Err(Error::StageNotImplemented(Stage::Parser))
    }
}
