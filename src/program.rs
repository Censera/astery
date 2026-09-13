use crate::parser::{
    BindingDeclaration, EnumDeclaration, FunctionDeclaration, Import, IntoImplementation,
    ModuleDeclaration, StructDeclaration, parse_bindings, parse_enums, parse_functions,
    parse_imports, parse_intos, parse_module, parse_structs,
};
use crate::shortcuts::{MacroDeclaration, parse_macros};
use crate::user_type::{UserTypeDeclaration, parse_user_types};
use crate::{Error, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Program {
    pub module: Option<ModuleDeclaration>,
    pub imports: Vec<Import>,
    pub enums: Vec<EnumDeclaration>,
    pub structs: Vec<StructDeclaration>,
    pub user_types: Vec<UserTypeDeclaration>,
    pub intos: Vec<IntoImplementation>,
    pub bindings: Vec<BindingDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub macros: Vec<MacroDeclaration>,
}

pub fn parse_program(tokens: &[Token]) -> Result<Program, Error> {
    let mut program = Program::default();
    let mut position = 0;

    while position < tokens.len() {
        let end = top_level_item_end(tokens, position)?;
        let item = &tokens[position..end];
        parse_item(item, &mut program)?;
        position = end;
    }

    Ok(program)
}

fn parse_item(tokens: &[Token], program: &mut Program) -> Result<(), Error> {
    let kind = first_declaration_kind(tokens)?;

    match kind {
        DeclarationKind::Module => {
            if program.module.is_some() {
                return Err(error_at(tokens.first(), "multiple module declarations"));
            }
            program.module = Some(parse_module(tokens)?);
        }
        DeclarationKind::Import => {
            program.imports.extend(parse_imports(tokens)?);
        }
        DeclarationKind::Enum => {
            program.enums.extend(parse_enums(tokens)?);
        }
        DeclarationKind::Struct => {
            program.structs.extend(parse_structs(tokens)?);
        }
        DeclarationKind::UserType => {
            program.user_types.extend(parse_user_types(tokens)?);
        }
        DeclarationKind::Into => {
            program.intos.extend(parse_intos(tokens)?);
        }
        DeclarationKind::Binding => {
            program.bindings.extend(parse_bindings(tokens)?);
        }
        DeclarationKind::Function => {
            program.functions.extend(parse_functions(tokens)?);
        }
        DeclarationKind::Macro => {
            program.macros.extend(parse_macros(tokens)?);
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclarationKind {
    Module,
    Import,
    Enum,
    Struct,
    UserType,
    Into,
    Binding,
    Function,
    Macro,
}

fn first_declaration_kind(tokens: &[Token]) -> Result<DeclarationKind, Error> {
    let index = skip_modifiers(tokens, 0);

    match tokens.get(index).map(Token::kind) {
        Some(TokenKind::Mod) => Ok(DeclarationKind::Module),
        Some(TokenKind::Use) => Ok(DeclarationKind::Import),
        Some(TokenKind::Enum) => Ok(DeclarationKind::Enum),
        Some(TokenKind::Struct) => Ok(DeclarationKind::Struct),
        Some(TokenKind::Type) => Ok(DeclarationKind::UserType),
        Some(TokenKind::Into) => Ok(DeclarationKind::Into),
        Some(TokenKind::Let | TokenKind::Const) => Ok(DeclarationKind::Binding),
        Some(TokenKind::Fn) => Ok(DeclarationKind::Function),
        Some(TokenKind::Macro) => Ok(DeclarationKind::Macro),
        _ => Err(error_at(
            tokens.get(index).or_else(|| tokens.last()),
            "unexpected top-level declaration",
        )),
    }
}

fn top_level_item_end(tokens: &[Token], start: usize) -> Result<usize, Error> {
    let position = skip_modifiers(tokens, start);

    match tokens.get(position).map(Token::kind) {
        Some(TokenKind::Mod) => Ok((position + 2).min(tokens.len())),
        Some(TokenKind::Use) => scan_until_statement_end(tokens, start, position + 1),
        Some(TokenKind::Let | TokenKind::Const) => {
            scan_until_statement_end(tokens, start, position + 1)
        }
        Some(TokenKind::Macro | TokenKind::Enum | TokenKind::Struct | TokenKind::Into | TokenKind::Fn) => {
            scan_braced_declaration(tokens, start)
        }
        Some(TokenKind::Type) => {
            let name = position + 2;
            if tokens.get(name).map(Token::kind) == Some(&TokenKind::Struct) {
                scan_braced_declaration(tokens, start)
            } else {
                scan_until_statement_end(tokens, start, name)
            }
        }
        _ => Err(error_at(
            tokens.get(position).or_else(|| tokens.last()),
            "unexpected top-level declaration",
        )),
    }
}

fn skip_modifiers(tokens: &[Token], mut position: usize) -> usize {
    loop {
        match tokens.get(position).map(Token::kind) {
            Some(TokenKind::Pub | TokenKind::Pri) => position += 1,
            Some(TokenKind::At) => {
                position += 1;
                if matches!(tokens.get(position).map(Token::kind), Some(TokenKind::Identifier(_))) {
                    position += 1;
                }
            }
            _ => return position,
        }
    }
}

fn scan_braced_declaration(tokens: &[Token], start: usize) -> Result<usize, Error> {
    let mut position = start;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;

    while let Some(token) = tokens.get(position) {
        match token.kind() {
            TokenKind::OpenParen => paren_depth += 1,
            TokenKind::CloseParen if paren_depth > 0 => paren_depth -= 1,
            TokenKind::OpenBracket => bracket_depth += 1,
            TokenKind::CloseBracket if bracket_depth > 0 => bracket_depth -= 1,
            TokenKind::OpenAngle if paren_depth == 0 && bracket_depth == 0 => angle_depth += 1,
            TokenKind::CloseAngle if angle_depth > 0 => angle_depth -= 1,
            TokenKind::OpenBrace
                if paren_depth == 0 && bracket_depth == 0 && angle_depth == 0 =>
            {
                return find_matching_brace(tokens, position).map(|end| end + 1);
            }
            _ => {}
        }
        position += 1;
    }

    Err(error_at(tokens.last(), "expected `{` in top-level declaration"))
}

fn find_matching_brace(tokens: &[Token], open: usize) -> Result<usize, Error> {
    let mut depth = 0usize;
    for position in open..tokens.len() {
        match tokens[position].kind() {
            TokenKind::OpenBrace => depth += 1,
            TokenKind::CloseBrace => {
                depth -= 1;
                if depth == 0 {
                    return Ok(position);
                }
            }
            _ => {}
        }
    }
    Err(error_at(tokens.last(), "unterminated top-level declaration"))
}

fn scan_until_statement_end(
    tokens: &[Token],
    start: usize,
    position: usize,
) -> Result<usize, Error> {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut current = position;

    while current < tokens.len() {
        match tokens[current].kind() {
            TokenKind::OpenParen => paren_depth += 1,
            TokenKind::CloseParen if paren_depth > 0 => paren_depth -= 1,
            TokenKind::OpenBracket => bracket_depth += 1,
            TokenKind::CloseBracket if bracket_depth > 0 => bracket_depth -= 1,
            TokenKind::OpenBrace => brace_depth += 1,
            TokenKind::CloseBrace if brace_depth > 0 => brace_depth -= 1,
            TokenKind::Semicolon
                if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 =>
            {
                return Ok(current + 1);
            }
            kind
                if current > start
                    && paren_depth == 0
                    && bracket_depth == 0
                    && brace_depth == 0
                    && is_top_level_declaration_start(kind) =>
            {
                return Ok(current);
            }
            _ => {}
        }
        current += 1;
    }

    Ok(tokens.len())
}

fn is_top_level_declaration_start(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Mod
            | TokenKind::Use
            | TokenKind::Let
            | TokenKind::Const
            | TokenKind::Enum
            | TokenKind::Struct
            | TokenKind::Type
            | TokenKind::Into
            | TokenKind::Fn
            | TokenKind::Macro
            | TokenKind::Pub
            | TokenKind::Pri
            | TokenKind::At
    )
}

fn error_at(token: Option<&Token>, message: &str) -> Error {
    let (line, column) = token
        .map(|token| (token.line(), token.column()))
        .unwrap_or((1, 1));
    Error::Parse {
        line,
        column,
        message: message.to_owned(),
    }
}
