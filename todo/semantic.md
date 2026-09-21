# Semantic TODO

## Parser boundary

- [ ] Define collection lengths and compile-time dimensions.
- [ ] Define which parser constructs are erased after semantic lowering and which survive to backend lowering.
- [ ] Ensure backend lowering never reparses source text.
- [ ] Associate every semantic declaration and reference with its source span.

## Modules and imports

- [ ] Detect duplicate module, import, and namespace declarations where forbidden.
- [ ] Resolve module paths and imported modules.
- [ ] Detect missing modules/imported items and ambiguous imported names.
- [ ] Enforce module visibility across module boundaries.
- [ ] Define module path lookup and duplicate-import behavior.
- [ ] Define importing, grouped imports, re-exports, and module-cycle behavior.

## Symbols and names

- [ ] Build symbol tables for module, function, implementation, and local scopes.
- [ ] Implement lexical block scopes where bindings are visible from their declaration to the end of the block and shadowing is allowed, matching the parser's scope stack.
- [ ] Resolve every identifier to exactly one declaration or report a precise error.
- [ ] Detect duplicate declarations.
- [ ] Resolve parameters, bindings, constants, functions, types, variants, fields, and methods through the correct namespace.
- [ ] Track the declaration behind every semantic reference.
- [ ] Keep lookup order deterministic.
- [ ] Report a name declared with more than one kind (type, function, value) and list the conflicting declarations.

## Types

- [ ] Resolve primitive, user-defined, alias, struct, enum, union, pointer, array, vector, and tuple types.
- [ ] Resolve parameterized union aliases and concrete type arguments.
- [ ] Detect unknown, duplicate, conflicting, malformed, and unsupported recursive types.
- [ ] Define type identity, equality, assignability, compatibility, and implicit-conversion rules.
- [ ] Define `None`, optional pointers, layout-relevant properties, and compile-time dimensions.
- [ ] Fix `T<length>` vector types: `type_syntax.rs` matches `OpenAngle` and `CloseAngle`, which the lexer never emits.
- [ ] Decide whether `i128` and `u128` are language types; `type_syntax.rs` accepts them and the type table in `doc/syntax.md` lists only `i8` to `i64` and `u8` to `u64`.
- [ ] Define the unit type used by functions without a return type and by blocks that end in `;`.

## Literals

- [ ] Infer integer, float, boolean, character, string, and `None` types.
- [ ] Validate literal ranges and target-type compatibility.
- [ ] Define literal inference and numeric conversion rules.

## Expressions and operators

- [ ] Type-check identifiers, literals, unary expressions, and binary expressions.
- [ ] Define arithmetic, comparison, logical, bitwise, shift, increment/decrement, and compound-assignment rules.
- [ ] Reject invalid operand combinations with both operand types in diagnostics.
- [ ] Resolve exactly one type for every successful expression.
- [ ] Define pointer, optional-pointer, collection, and string operator behavior.
- [ ] Validate short-circuiting and guaranteed evaluation order.

## Calls and overloads

- [ ] Resolve function declarations from call sites.
- [ ] Validate argument count and types.
- [ ] Resolve overloads by the documented signature rules.
- [ ] Define how the expected type selects among overloads that differ only by return type.
- [ ] Detect ambiguous and unmatched calls.
- [ ] Validate variadic and flagged calls, including `@Stripped` and `@Loosely`.
- [ ] Resolve methods after receiver type resolution.
- [ ] Detect duplicate/conflicting signatures and indistinguishable overloads.
- [ ] Preserve the selected declaration/signature for backend lowering.

## Bindings and assignment

- [ ] Type-check `let` and `const` bindings.
- [ ] Resolve inferred and explicit binding types.
- [ ] Validate multiple-binding and destructuring shapes.
- [ ] Enforce mutability and assignment rules.
- [ ] Define `_` discard behavior and initialization-before-use rules.
- [ ] Reject invalid self-referential initialization.
- [ ] Treat `let (a, b) = (b, a);` as new bindings that shadow the old ones.
- [ ] Type assignment as an expression that produces the assigned value, with a mutable local binding as the minimum assignable left-hand side.
- [ ] Process `let { ... }` and `const { ... }` lists in declaration order and reject a reference to a later binding in the same list.

## Blocks

- [ ] Type a block by its final expression when it has no `;`, and give it the unit type when it ends in `;` or holds no expression.
- [ ] Reject a block without a value in a context that requires one.
- [ ] Type the shortened forms `statement if condition;` and `value if condition else value`.

## Collections and ranges

- [ ] Validate array/vector construction and element consistency.
- [ ] Validate compile-time collection lengths.
- [ ] Resolve tuple construction and element types.
- [ ] Type-check indexing, index types, targets, ranges, inclusive ranges, and string slicing.
- [ ] Define result types and runtime out-of-bounds behavior.
- [ ] Reject statically provable invalid indices where required.
- [ ] Validate the repeat forms `T[length, value]` and `T<length, value>` and the inferred-length forms `T[]` and `T<>`.

## Structs, enums, unions, members

- [ ] Resolve and validate positional/named struct construction.
- [ ] Detect missing, unknown, and duplicate field initialization.
- [ ] Resolve member access and enforce field and variant visibility.
- [ ] Resolve enum variants and payloads.
- [ ] Resolve union values and member/variant access.
- [ ] Define unit, tuple, and field variants.
- [ ] Infer the union member from the value's type at construction, match by member type (`i64(value) {}`) instead of constructor name, and validate arms and exhaustiveness.

## `into` and methods

- [ ] Register `into` implementations.
- [ ] Resolve methods by receiver type and name.
- [ ] Enforce method visibility.
- [ ] Detect duplicate/conflicting methods.
- [ ] Validate method signatures and receiver/target relationships.
- [ ] Define method lookup precedence.
- [ ] Associate methods with implementations for backend lowering.

## Control flow

- [ ] Validate conditional, loop, `while`, `for`, and `match` conditions/scrutinees.
- [ ] Build loop contexts and resolve labeled `break`/`continue` targets.
- [ ] Reject invalid loop control targets.
- [ ] Validate match arms and coverage/exhaustiveness.
- [ ] Track reachability where required and define unreachable-code behavior.
- [ ] Define control-flow joins for binding state and type state.

## Returns and function bodies

- [ ] Validate every `return` against the enclosing function type.
- [ ] Reject invalid returns from `void` or non-void functions.
- [ ] Determine whether every non-void path returns.
- [ ] Validate final-expression return rules where supported.
- [ ] Validate function-entry parameters.

## Flags, macros, and embedded C

- [ ] Resolve supported `@flag` names and placement.
- [ ] Validate flag arguments and incompatible combinations.
- [ ] Enforce semantic effects of `@Stripped` and `@Loosely`.
- [ ] Rename the `Striped` and `Lossely` attributes and the `striped` and `lossely` flag strings to `Stripped` and `Loosely`, as `doc/syntax.md` spells them.

## Diagnostics and invariants

- [ ] Give every semantic failure a stable category and precise source span.
- [ ] Include relevant names/types in diagnostics.
- [ ] Avoid cascaded errors caused only by earlier unresolved declarations where recovery is unnecessary.
- [ ] Keep semantic analysis deterministic.
- [ ] Ensure successful programs contain no unresolved symbols or untyped expressions.
- [ ] Ensure successful calls, member accesses, and control-flow targets are fully resolved.
- [ ] Keep multi-source diagnostics and error ordering deterministic.

## Tests

- [ ] Test name resolution, shadowing, duplicates, imports, and visibility.
- [ ] Test primitive/user types, aliases, composites, unions, and pointers.
- [ ] Test literal typing and operator rules.
- [ ] Test calls, overloads, flags, methods, returns, assignment, and control flow.
- [ ] Test block values, assignment expressions, and multi-binding declaration order.
- [ ] Test macros, embedded C, deterministic diagnostics, and source locations.
