use crate::{Error, Token, TokenKind};

pub fn normalize_function_return_types(source: &str) -> Result<String, Error> {
    let tokens = crate::lexer::tokenize(source)?;
    let mut insertions = Vec::new();
    let line_starts = line_starts(source);

    let mut index = 0;
    while index < tokens.len() {
        if tokens[index].kind() != &TokenKind::Fn {
            index += 1;
            continue;
        }

        let first = index + 1;
        if first >= tokens.len() {
            break;
        }

        if tokens[first].kind() == &TokenKind::OpenBracket {
            index = first + 1;
            continue;
        }

        let Some(name_index) = find_function_name(&tokens, first) else {
            index = first;
            continue;
        };

        if name_index == first {
            index = name_index + 1;
            continue;
        }

        let start = offset(&line_starts, &tokens[first]);
        let end = offset(&line_starts, &tokens[name_index]);
        insertions.push((start, "["));
        insertions.push((end, "]"));
        index = name_index + 1;
    }

    let mut normalized = source.to_owned();
    for (position, text) in insertions.into_iter().rev() {
        normalized.insert_str(position, text);
    }
    Ok(normalized)
}

fn find_function_name(tokens: &[Token], start: usize) -> Option<usize> {
    let mut index = start;
    while index + 1 < tokens.len() {
        if matches!(tokens[index].kind(), TokenKind::Identifier(_))
            && tokens[index + 1].kind() == &TokenKind::OpenParen
        {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn line_starts(source: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (index, byte) in source.bytes().enumerate() {
        if byte == b'\n' {
            starts.push(index + 1);
        }
    }
    starts
}

fn offset(line_starts: &[usize], token: &Token) -> usize {
    line_starts[token.line() - 1] + token.column() - 1
}
