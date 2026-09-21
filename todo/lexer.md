# Lexer TODO

The lexer emits every v26 token except the new shift operators. Remaining work is the shift change, unused token kinds, and conformance and regression coverage.

## Implementation

- [ ] Lex `<:` as shift left and `:>` as shift right.
- [ ] Remove the `<<` and `>>` tokens so `name<name<index>>` lexes as `name < name < index > >`.
- [ ] Remove the `OpenAngle` and `CloseAngle` token kinds, which the lexer never emits, once `type_syntax.rs`, `user_type.rs`, and `program.rs` stop matching them.
- [ ] Decide whether `then` stays reserved and whether a bare `::` stays a token, since `doc/syntax.md` defines neither.

## Tests

- [ ] Test every keyword, including the logical keyword forms `and`, `or`, `xor`, and `not`.
- [ ] Test every operator and punctuation token, including `~~`, `^`, `?`, `:`, `::<`, `:&`, `:|`, `:^`, `:<`, `:>`, `<:`, `->`, `..`, `..=`, and `...`.
- [ ] Test that `<<` and `>>` are never produced.
- [ ] Test every literal form.
- [ ] Test valid escape sequences.
- [ ] Test loop labels against character literals (`'name` and `'a'`).
- [ ] Test `@` followed by an identifier for function flags.
- [ ] Test malformed literals and unterminated strings and characters.
- [ ] Test source locations for representative tokens.
- [ ] Add regression tests for every lexer bug fixed during v26.

## Conformance

- [ ] Compare lexer tokenization against every stable lexical rule in `doc/syntax.md`.
- [ ] Keep lexical errors deterministic and independent from parser/semantic failures.
