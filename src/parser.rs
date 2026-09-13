use crate::{Error, Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDeclaration {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub module: Option<String>,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportItem {
    pub name: String,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDeclaration {
    pub visibility: Option<Visibility>,
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariant {
    pub visibility: Option<Visibility>,
    pub name: String,
    pub kind: EnumVariantKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumVariantKind {
    Unit,
    Tuple(Vec<Vec<TokenKind>>),
    Fields(Vec<EnumField>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumField {
    pub name: String,
    pub type_tokens: Vec<TokenKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDeclaration {
    pub visibility: Option<Visibility>,
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub visibility: Option<Visibility>,
    pub name: String,
    pub type_tokens: Vec<TokenKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntoImplementation {
    pub target: String,
    pub methods: Vec<MethodDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDeclaration {
    pub visibility: Option<Visibility>,
    pub name: String,
    pub return_type: Vec<TokenKind>,
    pub parameters: Vec<Parameter>,
    pub flags: Vec<String>,
    pub body: Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    Let,
    Const,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingDeclaration {
    pub kind: BindingKind,
    pub bindings: Vec<Binding>,
    pub value: Option<Vec<TokenKind>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub name: String,
    pub type_tokens: Vec<TokenKind>,
    pub value: Option<Vec<TokenKind>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Vec<TokenKind>,
    pub parameters: Vec<Parameter>,
    pub flags: Vec<String>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub type_tokens: Vec<TokenKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Binding(BindingDeclaration),
    Return(Option<Expression>),
    Expression(Expression),
    If(IfStatement),
    Loop(LoopStatement),
    While(WhileStatement),
    For(ForStatement),
    Match(MatchStatement),
    Break(Option<String>),
    Continue(Option<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Block,
    pub elif_blocks: Vec<(Expression, Block)>,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopStatement {
    pub label: Option<String>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement {
    pub variable: String,
    pub iterable: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchStatement {
    pub value: Expression,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    pub pattern: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Integer(String),
    Float(String),
    String(String),
    Character(char),
    Boolean(bool),
    None,
    Identifier(String),
    Array(Vec<Expression>),
    Vector(Vec<Expression>),
    Tuple(Vec<Expression>),
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Member {
        value: Box<Expression>,
        name: String,
    },
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        inclusive: bool,
    },
    Unary {
        operator: UnaryOperator,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Positive,
    Negative,
    Not,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Or,
    Xor,
    And,
    BitOr,
    BitXor,
    BitAnd,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    ShiftLeft,
    ShiftRight,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn parse_module(tokens: &[Token]) -> Result<ModuleDeclaration, Error> {
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    parser.expect(TokenKind::Mod)?;
    Ok(ModuleDeclaration {
        name: parser.expect_name()?,
    })
}

pub fn parse_imports(tokens: &[Token]) -> Result<Vec<Import>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_imports()
}

pub fn parse_enums(tokens: &[Token]) -> Result<Vec<EnumDeclaration>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_enums()
}

pub fn parse_structs(tokens: &[Token]) -> Result<Vec<StructDeclaration>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_structs()
}

pub fn parse_intos(tokens: &[Token]) -> Result<Vec<IntoImplementation>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_intos()
}

pub fn parse_bindings(tokens: &[Token]) -> Result<Vec<BindingDeclaration>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_bindings()
}

pub fn parse_functions(tokens: &[Token]) -> Result<Vec<FunctionDeclaration>, Error> {
    Parser {
        tokens,
        position: 0,
    }
    .parse_functions()
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    fn parse_imports(mut self) -> Result<Vec<Import>, Error> {
        let mut imports = Vec::new();
        while self.peek_kind() == Some(&TokenKind::Use) {
            imports.push(self.parse_import()?);
        }
        Ok(imports)
    }

    fn parse_enums(mut self) -> Result<Vec<EnumDeclaration>, Error> {
        let mut declarations = Vec::new();
        while matches!(
            self.peek_kind(),
            Some(&TokenKind::Pub | &TokenKind::Pri | &TokenKind::Enum)
        ) {
            declarations.push(self.parse_enum()?);
        }
        if self.peek_kind().is_some() {
            return Err(self.error("unexpected token after enum"));
        }
        Ok(declarations)
    }

    fn parse_structs(mut self) -> Result<Vec<StructDeclaration>, Error> {
        let mut declarations = Vec::new();
        while matches!(
            self.peek_kind(),
            Some(&TokenKind::Pub | &TokenKind::Pri | &TokenKind::Struct)
        ) {
            declarations.push(self.parse_struct()?);
        }
        if self.peek_kind().is_some() {
            return Err(self.error("unexpected token after struct"));
        }
        Ok(declarations)
    }

    fn parse_intos(mut self) -> Result<Vec<IntoImplementation>, Error> {
        let mut implementations = Vec::new();
        while self.peek_kind() == Some(&TokenKind::Into) {
            implementations.push(self.parse_into()?);
        }
        if self.peek_kind().is_some() {
            return Err(self.error("unexpected token after into implementation"));
        }
        Ok(implementations)
    }

    fn parse_enum(&mut self) -> Result<EnumDeclaration, Error> {
        let visibility = self.parse_visibility();
        self.expect(TokenKind::Enum)?;
        let name = self.expect_binding_name()?;
        self.expect(TokenKind::OpenBrace)?;
        let mut variants = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated enum"));
            }
            variants.push(self.parse_enum_variant()?);
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in enum"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(EnumDeclaration {
            visibility,
            name,
            variants,
        })
    }

    fn parse_enum_variant(&mut self) -> Result<EnumVariant, Error> {
        let visibility = self.parse_visibility();
        let name = self.expect_binding_name()?;
        let kind = match self.peek_kind() {
            Some(TokenKind::OpenParen) => EnumVariantKind::Tuple(self.parse_enum_variant_types()?),
            Some(TokenKind::OpenBrace) => EnumVariantKind::Fields(self.parse_enum_fields()?),
            _ => EnumVariantKind::Unit,
        };
        Ok(EnumVariant {
            visibility,
            name,
            kind,
        })
    }

    fn parse_enum_variant_types(&mut self) -> Result<Vec<Vec<TokenKind>>, Error> {
        self.expect(TokenKind::OpenParen)?;
        if self.peek_kind() == Some(&TokenKind::CloseParen) {
            self.advance();
            return Ok(Vec::new());
        }
        let mut types = Vec::new();
        loop {
            let type_tokens = self
                .parse_type_tokens(|kind| matches!(kind, TokenKind::Comma | TokenKind::CloseParen));
            if type_tokens.is_empty() {
                return Err(self.error("expected enum variant type"));
            }
            types.push(type_tokens);
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        self.expect(TokenKind::CloseParen)?;
        Ok(types)
    }

    fn parse_enum_fields(&mut self) -> Result<Vec<EnumField>, Error> {
        self.expect(TokenKind::OpenBrace)?;
        if self.peek_kind() == Some(&TokenKind::CloseBrace) {
            return Err(self.error("enum variant requires at least one field"));
        }
        let mut fields = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated enum variant fields"));
            }
            let name = self.expect_binding_name()?;
            let type_tokens = self
                .parse_type_tokens(|kind| matches!(kind, TokenKind::Comma | TokenKind::CloseBrace));
            if type_tokens.is_empty() {
                return Err(self.error("expected enum field type"));
            }
            fields.push(EnumField { name, type_tokens });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in enum variant fields"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(fields)
    }

    fn parse_struct(&mut self) -> Result<StructDeclaration, Error> {
        let visibility = self.parse_visibility();
        self.expect(TokenKind::Struct)?;
        let name = self.expect_binding_name()?;
        self.expect(TokenKind::OpenBrace)?;
        let mut fields = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated struct"));
            }
            fields.push(self.parse_struct_field()?);
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in struct"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(StructDeclaration {
            visibility,
            name,
            fields,
        })
    }

    fn parse_struct_field(&mut self) -> Result<StructField, Error> {
        let visibility = self.parse_visibility();
        let name = self.expect_binding_name()?;
        let type_tokens =
            self.parse_type_tokens(|kind| matches!(kind, TokenKind::Comma | TokenKind::CloseBrace));
        if type_tokens.is_empty() {
            return Err(self.error("expected struct field type"));
        }
        Ok(StructField {
            visibility,
            name,
            type_tokens,
        })
    }

    fn parse_into(&mut self) -> Result<IntoImplementation, Error> {
        self.expect(TokenKind::Into)?;
        let target = self.expect_binding_name()?;
        self.expect(TokenKind::OpenBrace)?;
        let mut methods = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated into implementation"));
            }
            methods.push(self.parse_method()?);
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(IntoImplementation { target, methods })
    }

    fn parse_method(&mut self) -> Result<MethodDeclaration, Error> {
        let visibility = self.parse_visibility();
        let flags = self.parse_flags()?;
        self.expect(TokenKind::Fn)?;
        let return_type = if self.peek_kind() == Some(&TokenKind::OpenBracket) {
            self.parse_bracketed_tokens("expected method return type")?
        } else {
            Vec::new()
        };
        let name = self.expect_binding_name()?;
        self.expect(TokenKind::OpenParen)?;
        let parameters = self.parse_parameters()?;
        self.expect(TokenKind::CloseParen)?;
        let body = self.parse_block()?;
        Ok(MethodDeclaration {
            visibility,
            name,
            return_type,
            parameters,
            flags,
            body,
        })
    }

    fn parse_visibility(&mut self) -> Option<Visibility> {
        match self.peek_kind() {
            Some(TokenKind::Pub) => {
                self.advance();
                Some(Visibility::Public)
            }
            Some(TokenKind::Pri) => {
                self.advance();
                Some(Visibility::Private)
            }
            _ => None,
        }
    }

    fn parse_bindings(mut self) -> Result<Vec<BindingDeclaration>, Error> {
        let mut declarations = Vec::new();
        while matches!(self.peek_kind(), Some(&TokenKind::Let | &TokenKind::Const)) {
            declarations.push(self.parse_binding_declaration()?);
        }
        if self.peek_kind().is_some() {
            return Err(self.error("unexpected token after binding"));
        }
        Ok(declarations)
    }

    fn parse_functions(mut self) -> Result<Vec<FunctionDeclaration>, Error> {
        let mut declarations = Vec::new();
        while self.peek_kind().is_some_and(|kind| {
            matches!(kind, TokenKind::Fn | TokenKind::At)
        }) {
            declarations.push(self.parse_function()?);
        }
        if self.peek_kind().is_some() {
            return Err(self.error("unexpected token after function"));
        }
        Ok(declarations)
    }

    fn parse_import(&mut self) -> Result<Import, Error> {
        self.expect(TokenKind::Use)?;
        if self.peek_kind() == Some(&TokenKind::OpenBrace) {
            return Ok(Import {
                module: None,
                items: self.parse_import_items()?,
            });
        }
        let module = self.expect_name()?;
        let items = if self.peek_kind() == Some(&TokenKind::OpenBrace) {
            self.parse_import_items()?
        } else {
            Vec::new()
        };
        Ok(Import {
            module: Some(module),
            items,
        })
    }

    fn parse_import_items(&mut self) -> Result<Vec<ImportItem>, Error> {
        self.expect(TokenKind::OpenBrace)?;
        let mut items = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated import list"));
            }
            let name = self.expect_name()?;
            let nested = if self.peek_kind() == Some(&TokenKind::OpenBrace) {
                self.parse_import_items()?
            } else {
                Vec::new()
            };
            items.push(ImportItem { name, items: nested });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in import list"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(items)
    }

    fn parse_binding_declaration(&mut self) -> Result<BindingDeclaration, Error> {
        let kind = match self.advance_kind()? {
            TokenKind::Let => BindingKind::Let,
            TokenKind::Const => BindingKind::Const,
            _ => unreachable!(),
        };
        let bindings = if self.peek_kind() == Some(&TokenKind::OpenBrace) {
            self.parse_binding_block()?
        } else {
            self.parse_binding_list()?
        };
        let value = if self.peek_kind() == Some(&TokenKind::EqualSign) {
            self.advance();
            Some(self.collect_until(|kind| kind == &TokenKind::Semicolon))
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(BindingDeclaration { kind, bindings, value })
    }

    fn parse_binding_block(&mut self) -> Result<Vec<Binding>, Error> {
        self.expect(TokenKind::OpenBrace)?;
        let mut bindings = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated binding block"));
            }
            let name = self.expect_binding_name()?;
            let type_tokens = self.parse_binding_type();
            self.expect(TokenKind::EqualSign)?;
            let value = self.collect_until(|kind| matches!(kind, TokenKind::Comma | TokenKind::CloseBrace));
            if value.is_empty() {
                return Err(self.error("expected binding value"));
            }
            bindings.push(Binding {
                name,
                type_tokens,
                value: Some(value),
            });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in binding block"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(bindings)
    }

    fn parse_binding_list(&mut self) -> Result<Vec<Binding>, Error> {
        let mut names = Vec::new();
        loop {
            names.push(self.expect_binding_name()?);
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        let type_tokens = self.parse_binding_type();
        Ok(names
            .into_iter()
            .map(|name| Binding {
                name,
                type_tokens: type_tokens.clone(),
                value: None,
            })
            .collect())
    }

    fn parse_binding_type(&mut self) -> Vec<TokenKind> {
        self.parse_type_tokens(|kind| matches!(kind, TokenKind::EqualSign | TokenKind::Semicolon))
    }

    fn parse_function(&mut self) -> Result<FunctionDeclaration, Error> {
        let flags = self.parse_flags()?;
        self.expect(TokenKind::Fn)?;
        let return_type = if self.peek_kind() == Some(&TokenKind::OpenBracket) {
            self.parse_bracketed_tokens("expected function return type")?
        } else {
            Vec::new()
        };
        let name = self.expect_binding_name()?;
        self.expect(TokenKind::OpenParen)?;
        let parameters = self.parse_parameters()?;
        self.expect(TokenKind::CloseParen)?;
        let body = self.parse_block()?;
        Ok(FunctionDeclaration {
            name,
            return_type,
            parameters,
            flags,
            body,
        })
    }

    fn parse_flags(&mut self) -> Result<Vec<String>, Error> {
        let mut flags = Vec::new();
        while self.peek_kind() == Some(&TokenKind::At) {
            self.advance();
            flags.push(self.expect_name()?);
        }
        Ok(flags)
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, Error> {
        let mut parameters = Vec::new();
        if self.peek_kind() == Some(&TokenKind::CloseParen) {
            return Ok(parameters);
        }
        loop {
            if self.peek_kind() == Some(&TokenKind::Ellipsis) {
                self.advance();
                if self.peek_kind() != Some(&TokenKind::CloseParen) {
                    self.expect(TokenKind::Comma)?;
                }
                return Ok(parameters);
            }
            let name = self.expect_binding_name()?;
            let type_tokens = self.parse_type_tokens(|kind| matches!(kind, TokenKind::Comma | TokenKind::CloseParen));
            if type_tokens.is_empty() {
                return Err(self.error("expected parameter type"));
            }
            parameters.push(Parameter { name, type_tokens });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        Ok(parameters)
    }

    fn parse_block(&mut self) -> Result<Block, Error> {
        self.expect(TokenKind::OpenBrace)?;
        let mut statements = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            if self.peek_kind().is_none() {
                return Err(self.error("unterminated block"));
            }
            statements.push(self.parse_statement()?);
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.peek_kind() {
            Some(TokenKind::Let | TokenKind::Const) => Ok(Statement::Binding(self.parse_binding_declaration()?)),
            Some(TokenKind::Return) => self.parse_return(),
            Some(TokenKind::If) => self.parse_if_statement(),
            Some(TokenKind::Loop) => self.parse_loop_statement(),
            Some(TokenKind::While) => self.parse_while_statement(),
            Some(TokenKind::For) => self.parse_for_statement(),
            Some(TokenKind::Match) => self.parse_match_statement(),
            Some(TokenKind::Break) => self.parse_break_statement(),
            Some(TokenKind::Continue) => self.parse_continue_statement(),
            _ => {
                let expression = self.parse_expression()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Statement::Expression(expression))
            }
        }
    }

    fn parse_return(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::Return)?;
        if self.peek_kind() == Some(&TokenKind::CloseBrace) {
            return Ok(Statement::Return(None));
        }
        let value = self.parse_expression()?;
        Ok(Statement::Return(Some(value)))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::If)?;
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        let mut elif_blocks = Vec::new();
        while self.peek_kind() == Some(&TokenKind::Elif) {
            self.advance();
            let condition = self.parse_expression()?;
            let body = self.parse_block()?;
            elif_blocks.push((condition, body));
        }
        let else_block = if self.peek_kind() == Some(&TokenKind::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Statement::If(IfStatement {
            condition,
            body,
            elif_blocks,
            else_block,
        }))
    }

    fn parse_loop_statement(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::Loop)?;
        let label = if let Some(TokenKind::Label(label)) = self.peek_kind().cloned() {
            self.advance();
            Some(label)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(Statement::Loop(LoopStatement { label, body }))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::While)?;
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Statement::While(WhileStatement { condition, body }))
    }

    fn parse_for_statement(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::For)?;
        let variable = self.expect_binding_name()?;
        self.expect(TokenKind::In)?;
        let iterable = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(Statement::For(ForStatement { variable, iterable, body }))
    }

    fn parse_match_statement(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::Match)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::OpenBrace)?;
        let mut arms = Vec::new();
        while self.peek_kind() != Some(&TokenKind::CloseBrace) {
            let pattern = self.parse_expression()?;
            let body = self.parse_block()?;
            arms.push(MatchArm { pattern, body });
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
            } else if self.peek_kind() != Some(&TokenKind::CloseBrace) {
                return Err(self.error("expected `,` or `}` in match"));
            }
        }
        self.expect(TokenKind::CloseBrace)?;
        Ok(Statement::Match(MatchStatement { value, arms }))
    }

    fn parse_break_statement(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::Break)?;
        let label = if let Some(TokenKind::Label(label)) = self.peek_kind().cloned() {
            self.advance();
            Some(label)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Statement::Break(label))
    }

    fn parse_continue_statement(&mut self) -> Result<Statement, Error> {
        self.expect(TokenKind::Continue)?;
        let label = if let Some(TokenKind::Label(label)) = self.peek_kind().cloned() {
            self.advance();
            Some(label)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Statement::Continue(label))
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> Result<Expression, Error> {
        let mut left = self.parse_unary_expression()?;
        loop {
            if self.peek_kind() == Some(&TokenKind::Greater) && self.greater_terminates_expression() {
                break;
            }
            let Some((operator, precedence)) = self.peek_binary_operator() else {
                break;
            };
            if precedence < min_precedence {
                break;
            }
            self.advance();
            let right = self.parse_binary_expression(precedence + 1)?;
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, Error> {
        let operator = match self.peek_kind() {
            Some(TokenKind::Add) => Some(UnaryOperator::Positive),
            Some(TokenKind::Subtract) => Some(UnaryOperator::Negative),
            Some(TokenKind::Exclamation) => Some(UnaryOperator::Not),
            Some(TokenKind::BitNot) => Some(UnaryOperator::BitNot),
            _ => None,
        };
        if let Some(operator) = operator {
            self.advance();
            let value = self.parse_unary_expression()?;
            return Ok(Expression::Unary {
                operator,
                value: Box::new(value),
            });
        }
        self.parse_postfix_expression()
    }

    fn parse_postfix_expression(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_primary_expression()?;
        loop {
            match self.peek_kind() {
                Some(TokenKind::OpenParen) => {
                    self.advance();
                    let arguments = self.parse_arguments()?;
                    self.expect(TokenKind::CloseParen)?;
                    expression = Expression::Call {
                        function: Box::new(expression),
                        arguments,
                    };
                }
                Some(TokenKind::Dot) => {
                    self.advance();
                    let name = self.expect_name()?;
                    expression = Expression::Member {
                        value: Box::new(expression),
                        name,
                    };
                }
                Some(TokenKind::OpenBracket) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenKind::CloseBracket)?;
                    expression = Expression::Call {
                        function: Box::new(Expression::Member {
                            value: Box::new(expression),
                            name: "index".to_owned(),
                        }),
                        arguments: vec![index],
                    };
                }
                Some(TokenKind::OpenAngle) if self.can_parse_vector_literal() => {
                    self.advance();
                    let value = self.parse_expression_list(TokenKind::CloseAngle)?;
                    self.expect(TokenKind::CloseAngle)?;
                    expression = Expression::Vector(value);
                }
                _ => break,
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, Error> {
        match self.advance_kind()? {
            TokenKind::Integer(value) => Ok(Expression::Integer(value)),
            TokenKind::Float(value) => Ok(Expression::Float(value)),
            TokenKind::String(value) => Ok(Expression::String(value)),
            TokenKind::Character(value) => Ok(Expression::Character(value)),
            TokenKind::True => Ok(Expression::Boolean(true)),
            TokenKind::False => Ok(Expression::Boolean(false)),
            TokenKind::None => Ok(Expression::None),
            TokenKind::Identifier(value) => Ok(Expression::Identifier(value)),
            TokenKind::OpenParen => self.parse_parenthesized_expression(),
            TokenKind::OpenBracket => {
                let values = self.parse_expression_list(TokenKind::CloseBracket)?;
                self.expect(TokenKind::CloseBracket)?;
                Ok(Expression::Array(values))
            }
            kind => Err(self.error_with_kind("expected expression", kind)),
        }
    }

    fn parse_parenthesized_expression(&mut self) -> Result<Expression, Error> {
        if self.peek_kind() == Some(&TokenKind::CloseParen) {
            self.advance();
            return Ok(Expression::Tuple(Vec::new()));
        }
        let first = self.parse_expression()?;
        if self.peek_kind() != Some(&TokenKind::Comma) {
            self.expect(TokenKind::CloseParen)?;
            return Ok(first);
        }
        let mut values = vec![first];
        while self.peek_kind() == Some(&TokenKind::Comma) {
            self.advance();
            if self.peek_kind() == Some(&TokenKind::CloseParen) {
                break;
            }
            values.push(self.parse_expression()?);
        }
        self.expect(TokenKind::CloseParen)?;
        Ok(Expression::Tuple(values))
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, Error> {
        if self.peek_kind() == Some(&TokenKind::CloseParen) {
            return Ok(Vec::new());
        }
        let mut arguments = Vec::new();
        loop {
            arguments.push(self.parse_expression()?);
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        Ok(arguments)
    }

    fn parse_expression_list(&mut self, terminator: TokenKind) -> Result<Vec<Expression>, Error> {
        if self.peek_kind() == Some(&terminator) {
            return Ok(Vec::new());
        }
        let mut values = Vec::new();
        loop {
            values.push(self.parse_expression()?);
            if self.peek_kind() == Some(&TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        Ok(values)
    }

    fn can_parse_vector_literal(&self) -> bool {
        matches!(self.tokens.get(self.position + 1), Some(Token { kind: TokenKind::CloseAngle, .. }))
            || matches!(self.tokens.get(self.position + 1), Some(Token { kind: TokenKind::Integer(_), .. }))
            || matches!(self.tokens.get(self.position + 1), Some(Token { kind: TokenKind::Float(_), .. }))
            || matches!(self.tokens.get(self.position + 1), Some(Token { kind: TokenKind::String(_), .. }))
            || matches!(self.tokens.get(self.position + 1), Some(Token { kind: TokenKind::Character(_), .. }))
            || matches!(self.tokens.get(self.position + 1), Some(Token { kind: TokenKind::True | TokenKind::False | TokenKind::None | TokenKind::Identifier(_), .. }))
    }

    fn greater_terminates_expression(&self) -> bool {
        matches!(
            self.tokens.get(self.position + 1).map(Token::kind),
            Some(
                TokenKind::Comma
                    | TokenKind::Semicolon
                    | TokenKind::CloseParen
                    | TokenKind::CloseBracket
                    | TokenKind::CloseBrace
                    | TokenKind::Greater
            )
        )
    }

    fn peek_binary_operator(&self) -> Option<(BinaryOperator, u8)> {
        match self.peek_kind()? {
            TokenKind::Or => Some((BinaryOperator::Or, 1)),
            TokenKind::Xor => Some((BinaryOperator::Xor, 2)),
            TokenKind::And => Some((BinaryOperator::And, 3)),
            TokenKind::BitOr => Some((BinaryOperator::BitOr, 4)),
            TokenKind::BitXor => Some((BinaryOperator::BitXor, 5)),
            TokenKind::BitAnd => Some((BinaryOperator::BitAnd, 6)),
            TokenKind::Equal => Some((BinaryOperator::Equal, 7)),
            TokenKind::NotEqual => Some((BinaryOperator::NotEqual, 7)),
            TokenKind::Greater => Some((BinaryOperator::Greater, 8)),
            TokenKind::GreaterEqual => Some((BinaryOperator::GreaterEqual, 8)),
            TokenKind::Less => Some((BinaryOperator::Less, 8)),
            TokenKind::LessEqual => Some((BinaryOperator::LessEqual, 8)),
            TokenKind::ShiftLeft => Some((BinaryOperator::ShiftLeft, 9)),
            TokenKind::ShiftRight => Some((BinaryOperator::ShiftRight, 9)),
            TokenKind::Add => Some((BinaryOperator::Add, 10)),
            TokenKind::Subtract => Some((BinaryOperator::Subtract, 10)),
            TokenKind::Multiply => Some((BinaryOperator::Multiply, 11)),
            TokenKind::Divide => Some((BinaryOperator::Divide, 11)),
            _ => None,
        }
    }

    fn parse_type_tokens<F>(&mut self, stop: F) -> Vec<TokenKind>
    where
        F: Fn(&TokenKind) -> bool,
    {
        self.collect_until(stop)
    }

    fn parse_bracketed_tokens(&mut self, message: &str) -> Result<Vec<TokenKind>, Error> {
        self.expect(TokenKind::OpenBracket)?;
        let values = self.collect_until(|kind| kind == &TokenKind::CloseBracket);
        if values.is_empty() {
            return Err(self.error(message));
        }
        self.expect(TokenKind::CloseBracket)?;
        Ok(values)
    }

    fn expect_binding_name(&mut self) -> Result<String, Error> {
        let name = self.expect_name()?;
        Ok(name)
    }

    fn expect_name(&mut self) -> Result<String, Error> {
        match self.peek_kind() {
            Some(TokenKind::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            Some(TokenKind::Label(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(self.error("expected identifier")),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), Error> {
        if self.peek_kind() == Some(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("expected {kind:?}")))
        }
    }

    fn advance_kind(&mut self) -> Result<TokenKind, Error> {
        let Some(token) = self.tokens.get(self.position) else {
            return Err(self.error("unexpected end of input"));
        };
        let kind = token.kind().clone();
        self.position += 1;
        Ok(kind)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek_kind(&self) -> Option<&TokenKind> {
        self.tokens.get(self.position).map(Token::kind)
    }

    fn collect_until<F>(&mut self, stop: F) -> Vec<TokenKind>
    where
        F: Fn(&TokenKind) -> bool,
    {
        let start = self.position;
        while let Some(kind) = self.peek_kind() {
            if stop(kind) {
                break;
            }
            self.advance();
        }
        self.tokens[start..self.position]
            .iter()
            .map(Token::kind)
            .cloned()
            .collect()
    }

    fn error(&self, message: &str) -> Error {
        let token = self
            .tokens
            .get(self.position)
            .or_else(|| self.tokens.last());
        let (line, column) = token
            .map(|token| (token.line(), token.column()))
            .unwrap_or((1, 1));
        Error::Parse {
            line,
            column,
            message: message.to_owned(),
        }
    }

    fn error_with_kind(&self, message: &str, _kind: TokenKind) -> Error {
        self.error(message)
    }
}

fn _keep_parser_tests_imports_used(
    _: BinaryOperator,
    _: EnumVariantKind,
    _: Expression,
    _: IntoImplementation,
    _: Statement,
    _: StructDeclaration,
    _: StructField,
    _: Visibility,
) {
}

#[cfg(test)]
mod tests {
    use super::{
        BinaryOperator, EnumVariantKind, Expression, IntoImplementation, Statement,
        StructDeclaration, StructField, Visibility, parse_enums, parse_functions, parse_intos,
        parse_structs,
    };
    use crate::{TokenKind, lexer::tokenize};

    fn parse(source: &str) -> Vec<Statement> {
        let tokens = tokenize(source).unwrap();
        parse_functions(&tokens).unwrap().remove(0).body.statements
    }

    #[test]
    fn parses_enums() {
        let tokens = tokenize("pub enum Result { ok, pri error(i32), value { data string } }").unwrap();
        let enums = parse_enums(&tokens).unwrap();
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].visibility, Some(Visibility::Public));
        assert_eq!(enums[0].name, "Result");
        assert_eq!(enums[0].variants.len(), 3);
        assert_eq!(enums[0].variants[0].kind, EnumVariantKind::Unit);
        assert_eq!(enums[0].variants[1].visibility, Some(Visibility::Private));
        assert_eq!(
            enums[0].variants[1].kind,
            EnumVariantKind::Tuple(vec![vec![TokenKind::Identifier("i32".into())]])
        );
        assert_eq!(
            enums[0].variants[2].kind,
            EnumVariantKind::Fields(vec![super::EnumField {
                name: "data".into(),
                type_tokens: vec![TokenKind::Identifier("string".into())],
            }])
        );
    }

    #[test]
    fn parses_structs() {
        let tokens = tokenize("pub struct Point { pub x f64, pri y f64 } struct Empty {}").unwrap();
        let structs = parse_structs(&tokens).unwrap();
        assert_eq!(structs.len(), 2);
        assert_eq!(structs[0].visibility, Some(Visibility::Public));
        assert_eq!(structs[0].name, "Point");
        assert_eq!(structs[0].fields.len(), 2);
        assert_eq!(structs[0].fields[0].visibility, Some(Visibility::Public));
        assert_eq!(structs[0].fields[1].visibility, Some(Visibility::Private));
        assert_eq!(structs[1].name, "Empty");
    }

    #[test]
    fn parses_intos() {
        let tokens = tokenize("into Point { pub fn [i32] x() {} pri fn y(value i64) {} }").unwrap();
        let implementations = parse_intos(&tokens).unwrap();
        assert_eq!(implementations.len(), 1);
        assert_eq!(implementations[0].target, "Point");
        assert_eq!(implementations[0].methods.len(), 2);
        assert_eq!(implementations[0].methods[0].visibility, Some(Visibility::Public));
        assert_eq!(implementations[0].methods[0].name, "x");
        assert_eq!(implementations[0].methods[0].return_type, vec![TokenKind::Identifier("i32".into())]);
        assert_eq!(implementations[0].methods[1].visibility, Some(Visibility::Private));
        assert_eq!(implementations[0].methods[1].name, "y");
        assert_eq!(implementations[0].methods[1].parameters.len(), 1);
    }
}
