# Parser TODO

The parser handles module and import declarations, enums, structs, `into` blocks, user types, bindings, functions, and the control-flow statements. The expression grammar, block values, and name-kind parsing that `doc/syntax.md` specifies are missing.

## Blocks and expressions

- [ ] Preserve whether a block's final expression ended in `;`, because it decides whether the block has a value.
- [ ] Parse block expressions wherever an expression is expected.
- [ ] Parse assignment as an expression, including `+=`, `-=`, `*=`, `/=`, `++`, and `--`.
- [ ] Parse the shortened forms `statement if condition;`, `break if condition else continue`, and `value if condition else value` in bindings.
- [ ] Parse indexing `name[index]`, vector indexing `name<index>`, and string slicing `"Hello"[1..4]`.
- [ ] Parse struct literals in positional and named form.
- [ ] Parse binding values as expressions instead of raw token lists.
- [ ] Parse bindings without an initializer (`let name T[length];`, `let (name, name) T;`).
- [ ] Parse tuple binding patterns with `_` slots (`let (a, b) = (b, a);`).
- [ ] Decide whether the parser or `type_syntax.rs` owns type parsing; the AST stores raw type tokens for semantic analysis to reparse.

## Name kinds

- [ ] Build a `NameIndex` per source unit that maps each name to one kind (Type, Function, Callable value, Vector, Value) and records `into` functions as members of their type, using ordered maps.
- [ ] Split each unit's expanded token stream into top-level items with kind, token range, and modifiers in one function.
- [ ] Record function flags and parameter counts in the index, and record an overloaded name once as a function.
- [ ] Keep macro names in a separate table.
- [ ] Skip items that fail to scan without reporting an error, and leave the error to the parser.
- [ ] Compile in two passes over `Sources`: index every unit, then parse each unit with its own index plus the names its `use` declarations select.
- [ ] Give `Parser` a stack of local scopes that carries kinds for parameters, `let`, `const`, brace lists, tuple patterns, `for` variables, and match-arm bindings.
- [ ] Read `{` after a Type as a struct literal, and after a Function or Callable value as a chain of block-argument calls.
- [ ] Read `<` after a Vector as an index and after any other name as a comparison.
- [ ] Support brace-argument calls on `Type.function` paths for functions declared in `into` blocks.
- [ ] Parse `@Stripped` calls using the recorded parameter count and `@Loosely` calls up to the end of the enclosing expression.
- [ ] Reject stripped calls to a function whose overloads differ in parameter count.
- [ ] Report a brace form used on an ambiguous name.
- [ ] Report a condition that ends in a brace-argument call with the documented message, for example "`check` is a function, so `{` was read as its argument; write `check()` to call it."

## Tests

- [ ] Test every documented declaration form.
- [ ] Test every documented expression form.
- [ ] Test every control-flow form.
- [ ] Test arrays, vectors, tuples, structs, enums, unions, pointers, casts, macros, imports, and embedded C.
- [ ] Test overload declarations.
- [ ] Test visibility declarations.
- [ ] Test function flags spelled `@Stripped` and `@Loosely`.
- [ ] Test every known parser rejection case.
- [ ] Test precedence and associativity with mixed expressions.
- [ ] Test block values with and without a final `;`.
- [ ] Test struct literal, block body, and block-argument readings of `{` for each name kind.
- [ ] Test `@Stripped` and `@Loosely` argument extents, including `g(f a, b)` and `g(print a, b)`.
- [ ] Test `name<index>` against `name < index`.
- [ ] Add regression tests for every parser bug fixed during v26.

## Conformance

- [ ] Compare parser behavior against every stable grammar rule in `doc/syntax.md`.
- [ ] Keep malformed nesting and delimiter diagnostics anchored to the smallest useful location.
