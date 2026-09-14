# Parser TODO

The parser implementation checklist is complete. Remaining work is conformance and regression coverage.

## Tests

- [ ] Test every documented declaration form.
- [ ] Test every documented expression form.
- [ ] Test every control-flow form.
- [ ] Test arrays, vectors, tuples, structs, enums, unions, pointers, casts, macros, imports, and embedded C.
- [ ] Test overload declarations.
- [ ] Test visibility declarations.
- [ ] Test function flags.
- [ ] Test every known parser rejection case.
- [ ] Test precedence and associativity with mixed expressions.
- [ ] Add regression tests for every parser bug fixed during v26.8.

## Conformance

- [ ] Compare parser behavior against every stable grammar rule in `doc/syntaxdesign-26.8.md`.
- [ ] Keep malformed nesting and delimiter diagnostics anchored to the smallest useful location.
