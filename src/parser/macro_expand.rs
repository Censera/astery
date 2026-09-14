use std::collections::HashMap;

use crate::shortcuts::{MacroDeclaration, parse_macros};
use crate::{Error, Token, TokenKind};

const MAX_EXPANSION_DEPTH: usize = 64;

pub fn expand_macros(tokens: &[Token]) -> Result<Vec<Token>, Error> {
    let spans = macro_declaration_spans(tokens)?;
    let mut definitions = HashMap::new();

    for (start, end) in &spans {
        let declarations = parse_macros(&tokens[*start..*end])?;
        for declaration in declarations {
            if definitions
                .insert(declaration.name.clone(), declaration)
                .is_some()
            {
                return Err(error_at(tokens.get(*start), "duplicate macro declaration"));
            }
        }
    }

    let mut source = Vec::with_capacity(tokens.len());
    let mut position = 0;
    for (start, end) in spans {
        source.extend_from_slice(&tokens[position..start]);
        position = end;
    }
    source.extend_from_slice(&tokens[position..]);

    expand_tokens(&source, &definitions, 0)
}

fn macro_declaration_spans(tokens: &[Token]) -> Result<Vec<(usize, usize)>, Error> {
    let mut spans = Vec::new();
    let mut position = 0;
    let mut brace_depth = 0usize;

    while position < tokens.len() {
        match tokens[position].kind() {
            TokenKind::OpenBrace => brace_depth += 1,
            TokenKind::CloseBrace if brace_depth > 0 => brace_depth -= 1,
            TokenKind::Macro if brace_depth == 0 => {
                let end = declaration_end(tokens, position)?;
                spans.push((position, end));
                position = end;
                continue;
            }
            _ => {}
        }
        position += 1;
    }

    Ok(spans)
}

fn declaration_end(tokens: &[Token], start: usize) -> Result<usize, Error> {
    let mut position = start;
    let mut found_body = false;
    let mut brace_depth = 0usize;

    while let Some(token) = tokens.get(position) {
        match token.kind() {
            TokenKind::OpenBrace => {
                found_body = true;
                brace_depth += 1;
            }
            TokenKind::CloseBrace if brace_depth > 0 => {
                brace_depth -= 1;
                if found_body && brace_depth == 0 {
                    return Ok(position + 1);
                }
            }
            _ => {}
        }
        position += 1;
    }

    Err(error_at(
        tokens.get(start),
        "unterminated macro declaration",
    ))
}

fn expand_tokens(
    tokens: &[Token],
    definitions: &HashMap<String, MacroDeclaration>,
    depth: usize,
) -> Result<Vec<Token>, Error> {
    if depth >= MAX_EXPANSION_DEPTH {
        return Err(error_at(
            tokens.first(),
            "macro expansion exceeded the maximum depth",
        ));
    }

    let mut result = Vec::with_capacity(tokens.len());
    let mut position = 0;

    while position < tokens.len() {
        if let Some((name, argument_end)) = macro_call_at(tokens, position) {
            let definition = definitions.get(&name).ok_or_else(|| {
                error_at(tokens.get(position), &format!("unknown macro `{name}`"))
            })?;
            let (arguments, next) = parse_call_arguments(tokens, position, argument_end)?;

            if arguments.len() != definition.parameters.len() {
                return Err(error_at(
                    tokens.get(position),
                    &format!(
                        "macro `{name}` expects {} argument(s), found {}",
                        definition.parameters.len(),
                        arguments.len()
                    ),
                ));
            }

            let expanded = substitute(definition, &arguments);
            result.extend(expand_tokens(&expanded, definitions, depth + 1)?);
            position = next;
            continue;
        }

        result.push(tokens[position].clone());
        position += 1;
    }

    Ok(result)
}

fn macro_call_at(tokens: &[Token], position: usize) -> Option<(String, usize)> {
    let name = match tokens.get(position)?.kind() {
        TokenKind::Identifier(name) => name.clone(),
        _ => return None,
    };
    if !matches!(tokens.get(position + 1)?.kind(), TokenKind::Exclamation) {
        return None;
    }
    if !matches!(tokens.get(position + 2)?.kind(), TokenKind::OpenParen) {
        return None;
    }
    Some((name, position + 2))
}

fn parse_call_arguments(
    tokens: &[Token],
    start: usize,
    open_paren: usize,
) -> Result<(Vec<Vec<Token>>, usize), Error> {
    let mut arguments = Vec::new();
    let mut position = open_paren + 1;

    if matches!(
        tokens.get(position).map(Token::kind),
        Some(TokenKind::CloseParen)
    ) {
        return Ok((arguments, position + 1));
    }

    loop {
        let argument_start = position;
        let mut depth = 0usize;

        while let Some(token) = tokens.get(position) {
            match token.kind() {
                TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace => depth += 1,
                TokenKind::CloseParen | TokenKind::CloseBracket | TokenKind::CloseBrace => {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                }
                TokenKind::Comma if depth == 0 => break,
                _ => {}
            }
            position += 1;
        }

        if argument_start == position {
            return Err(error_at(tokens.get(position), "expected macro argument"));
        }
        arguments.push(tokens[argument_start..position].to_vec());

        match tokens.get(position).map(Token::kind) {
            Some(TokenKind::Comma) => {
                position += 1;
                if matches!(
                    tokens.get(position).map(Token::kind),
                    Some(TokenKind::CloseParen)
                ) {
                    return Err(error_at(tokens.get(position), "expected macro argument"));
                }
            }
            Some(TokenKind::CloseParen) => return Ok((arguments, position + 1)),
            _ => {
                return Err(error_at(
                    tokens.get(start),
                    "expected `,` or `)` in macro call",
                ));
            }
        }
    }
}

fn substitute(definition: &MacroDeclaration, arguments: &[Vec<Token>]) -> Vec<Token> {
    let mut result = Vec::new();

    for token in &definition.body {
        let parameter = match token.kind() {
            TokenKind::Identifier(name) => definition
                .parameters
                .iter()
                .position(|parameter| parameter == name),
            _ => None,
        };

        if let Some(index) = parameter {
            result.extend(arguments[index].iter().cloned());
        } else {
            result.push(token.clone());
        }
    }

    result
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

#[cfg(test)]
mod tests {
    use super::expand_macros;
    use crate::TokenKind;
    use crate::lexer::tokenize;

    fn kinds(source: &str) -> Vec<TokenKind> {
        expand_macros(&tokenize(source).unwrap())
            .unwrap()
            .into_iter()
            .map(|token| token.kind().clone())
            .collect()
    }

    #[test]
    fn expands_macro_calls() {
        assert_eq!(
            kinds("macro square(x) { x * x } square!(4)"),
            vec![
                TokenKind::Integer("4".into()),
                TokenKind::Multiply,
                TokenKind::Integer("4".into()),
            ]
        );
    }

    #[test]
    fn expands_nested_macros() {
        assert_eq!(
            kinds(
                "macro square(x) { x * x } macro double(x) { square!(x) + square!(x) } double!(3)"
            ),
            vec![
                TokenKind::Integer("3".into()),
                TokenKind::Multiply,
                TokenKind::Integer("3".into()),
                TokenKind::Add,
                TokenKind::Integer("3".into()),
                TokenKind::Multiply,
                TokenKind::Integer("3".into()),
            ]
        );
    }

    #[test]
    fn preserves_argument_locations() {
        let tokens = tokenize("macro identity(x) { x }\nidentity!(42)").unwrap();
        let expanded = expand_macros(&tokens).unwrap();
        assert_eq!(expanded[0].kind(), &TokenKind::Integer("42".into()));
        assert_eq!(expanded[0].line(), 2);
    }

    #[test]
    fn rejects_unknown_macros() {
        let error = expand_macros(&tokenize("missing!()").unwrap()).unwrap_err();
        assert!(error.to_string().contains("unknown macro `missing`"));
    }

    #[test]
    fn rejects_wrong_argument_count() {
        let error =
            expand_macros(&tokenize("macro pair(a, b) { a + b } pair!(1)").unwrap()).unwrap_err();
        assert!(error.to_string().contains("expects 2 argument(s), found 1"));
    }

    #[test]
    fn rejects_recursive_expansion() {
        let error = expand_macros(&tokenize("macro recurse() { recurse!() } recurse!()").unwrap())
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("macro expansion exceeded the maximum depth")
        );
    }
}
