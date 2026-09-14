# Lexer TODO

The lexer implementation checklist is complete. Remaining work is conformance and regression coverage.

## Tests

- [ ] Test every keyword.
- [ ] Test every operator and punctuation token, including `~~`, `^`, `:`, `::`, and `::<`.
- [ ] Test every literal form.
- [ ] Test valid escape sequences.
- [ ] Test malformed literals and unterminated strings, characters, comments, and embedded blocks.
- [ ] Test source locations for representative tokens.
- [ ] Add regression tests for every lexer bug fixed during v26.8.

## Conformance

- [ ] Compare lexer tokenization against every stable lexical rule in `doc/syntaxdesign-26.8.md`.
- [ ] Keep lexical errors deterministic and independent from parser/semantic failures.
