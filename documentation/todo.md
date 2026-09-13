# TODO

## Foundation

- [x] Define the v26 syntax source of truth.
- [x] Establish the compiler library and executable entry point.
- [x] Establish explicit compiler errors.
- [x] Keep compiler stage boundaries explicit.

## Lexer

- [x] Tokenize module declarations, identifiers, and keywords.
- [x] Tokenize primitive literals.
- [x] Tokenize strings and characters.
- [x] Tokenize strings and characters.
- [x] Tokenize operators and punctuation.
- [x] Tokenize visibility and function flags.
- [x] Tokenize pointer and cast syntax.
- [x] Tokenize ranges and inclusive ranges.
- [x] Tokenize collection delimiters and literals.
- [x] Tokenize macro syntax.
- [x] Tokenize embedded C blocks.
- [x] Tokenize union type syntax.

## Parser

- [x] Parse module declarations.
- [x] Parse imports.
- [x] Parse `let` and `const` bindings.
- [x] Parse functions and overload declarations.
- [x] Parse function flags.
- [x] Parse blocks and expressions.
- [x] Parse calls and member access.
- [x] Parse `if`, `elif`, `else`, `loop`, `while`, `for`, and `match`.
- [x] Parse shortened conditional statements and expressions.
- [x] Parse `break` and `continue` with labels.
- [x] Parse range and inclusive-range expressions.
- [x] Parse arrays, vectors, and tuples.
- [x] Parse enums and enum variants.
- [x] Parse structs and fields.
- [x] Parse `into` implementations.
- [x] Parse user-defined types.
- [x] Parse casts and pointers.
- [x] Parse embedded C blocks.
- [x] Parse macros and macro calls.

## Parser to semantic boundary

- [x] Accept the `fn T name()` return-type form through the compiler parser boundary.
- [x] Accept the same return-type form for `into` methods.
- [x] Establish one semantic type representation for primitive, user-defined, pointer, array, vector, tuple, and union types.
- [x] Make Semantic a real diagnostic stage instead of only a stage name.
- [x] Represent casts, address-of, and pointer expressions inside the normal semantic expression model.
- [ ] Expand macros before semantic analysis while preserving useful source locations.
- [x] Build one complete program/module parse unit for semantic analysis.
- [ ] Attach source spans to semantic inputs so type and name errors use their real source locations.

## Semantic analysis

- [ ] Resolve modules and imports.
- [ ] Resolve names.
- [ ] Resolve scopes and shadowing.
- [ ] Check visibility.
- [ ] Check primitive and user-defined types.
- [ ] Check arrays, vectors, tuples, and their indexing/access rules.
- [ ] Resolve overloaded functions.
- [ ] Resolve `into` implementations and methods.
- [ ] Check returns.
- [ ] Check casts.
- [ ] Check pointer and optional-pointer rules.
- [ ] Check control-flow targets and labels.
- [ ] Resolve struct and enum members.
- [ ] Validate function flags.
- [ ] Validate embedded C boundaries.

## Backend

- [ ] Define the Astery to `an-inkwell` boundary.
- [ ] Lower constants and primitive values.
- [ ] Lower local bindings.
- [ ] Lower arithmetic and logical operations.
- [ ] Lower comparisons.
- [ ] Lower ranges.
- [ ] Lower casts.
- [ ] Lower functions.
- [ ] Lower calls and method calls.
- [ ] Lower returns.
- [ ] Lower basic blocks.
- [ ] Lower branches and loops.
- [ ] Lower pointers, allocation, loads, and stores.
- [ ] Lower arrays, vectors, and tuples.
- [ ] Lower structs and enums.
