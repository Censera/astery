use crate::{Error, Token, TokenKind};

pub fn normalize_function_return_tokens(source: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = crate::lexer::tokenize(source)?;
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

        let (start_line, start_column) = {
            let token = &tokens[first];
            (token.line(), token.column())
        };
        let (name_line, name_column) = {
            let token = &tokens[name_index];
            (token.line(), token.column())
        };

        tokens.insert(
            first,
            Token::synthetic(TokenKind::OpenBracket, start_line, start_column),
        );
        let close_index = name_index + 1;
        tokens.insert(
            close_index,
            Token::synthetic(TokenKind::CloseBracket, name_line, name_column),
        );
        index = close_index + 1;
    }

    Ok(tokens)
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
