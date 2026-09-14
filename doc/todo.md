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
- [x] Expand macros before semantic analysis while preserving useful source locations.
- [x] Build one complete program/module parse unit for semantic analysis.
- [ ] Attach source spans to semantic inputs so type and name errors use their real source locations.

## Semantic analysis

### Semantic model and source ownership

- [x] Define the semantic program representation produced from the parser program.
- [x] Define source spans for every semantic input that can produce a diagnostic.
- [x] Preserve the original source location of declarations, identifiers, type references, literals, operators, expressions, statements, and macro-expanded input.
- [x] Make semantic errors point at the smallest useful source span instead of a generic file position.
- [x] Define the distinction between parser errors, semantic errors, and backend errors.
- [x] Define which parser constructs are discarded after semantic lowering and which information must remain for diagnostics and code generation.

### Modules, imports, and namespaces

- [x] Register the root module for each source unit.
- [ ] Resolve module paths and imported modules.
- [ ] Resolve imported names and grouped imports.
- [ ] Detect missing modules and missing imported items.
- [ ] Detect duplicate module, import, and namespace declarations where forbidden.
- [x] Define the namespace rules for functions, bindings, types, enum variants, struct members, and methods.
- [ ] Detect ambiguous names introduced by imports.
- [ ] Enforce module visibility across module boundaries.

### Symbols, scopes, and name resolution

- [ ] Build symbol tables for global and local declarations.
- [ ] Define lexical scope creation and destruction rules.
- [ ] Resolve identifiers to exactly one declaration or produce a precise unresolved-name error.
- [ ] Define shadowing behavior and validate legal shadowing.
- [ ] Detect duplicate declarations inside the same scope.
- [ ] Resolve parameters, local bindings, constants, functions, types, enum variants, struct fields, and methods through the correct namespace.
- [ ] Track the declaration associated with each resolved semantic reference.

### Types

- [ ] Resolve primitive types.
- [ ] Resolve user-defined types and aliases.
- [ ] Resolve structs, enums, and union types as concrete semantic types.
- [ ] Resolve pointer and optional-pointer types.
- [ ] Resolve array element types and compile-time lengths.
- [ ] Resolve vector element types and compile-time lengths.
- [ ] Resolve tuple element types.
- [ ] Detect unknown types.
- [ ] Detect duplicate or conflicting type declarations.
- [ ] Detect invalid recursive type definitions where the language representation cannot support them.
- [ ] Define type identity and equality rules.
- [ ] Define assignability and compatibility rules.
- [ ] Define when implicit conversions are allowed, if any.
- [ ] Define the semantic representation of `None` and optional pointers.

### Literals and value typing

- [ ] Infer the types of integer, floating-point, boolean, character, string, and `None` literals.
- [ ] Validate literal values against their target types and ranges.
- [ ] Define literal type inference when no explicit type is provided.
- [ ] Reject invalid literal-to-type assignments.

### Expressions and operators

- [ ] Type-check identifier expressions.
- [ ] Type-check literals.
- [ ] Type-check unary expressions.
- [ ] Type-check binary expressions.
- [ ] Define operand type rules for arithmetic operators.
- [ ] Define operand type rules for comparison operators.
- [ ] Define operand type rules for logical operators.
- [ ] Define operand type rules for bitwise operators.
- [ ] Define operand type rules for shifts.
- [ ] Define operand type rules for increment and decrement.
- [ ] Define compound-assignment rules.
- [ ] Reject invalid operand combinations with both operand types in the diagnostic.
- [ ] Resolve the resulting type of every expression.
- [ ] Ensure every semantic expression has exactly one well-defined type or an explicit error state.

### Calls, overloads, and function signatures

- [ ] Resolve function declarations from call sites.
- [ ] Validate argument count.
- [ ] Validate argument types.
- [ ] Resolve overloaded functions by their parameter types and applicable signature rules.
- [ ] Define how return type participates in overload identity and selection, matching the language specification.
- [ ] Detect ambiguous overload resolution.
- [ ] Detect calls with no matching overload.
- [ ] Validate variadic or flagged function call behavior.
- [ ] Resolve method calls after receiver type resolution.
- [ ] Validate function return types and parameter types.
- [ ] Detect duplicate or conflicting function signatures.

### Bindings, constants, and assignment

- [ ] Type-check `let` bindings.
- [ ] Type-check `const` bindings.
- [ ] Resolve inferred binding types from initializer expressions.
- [ ] Validate explicitly declared binding types against initializers.
- [ ] Validate multiple-binding declarations and destructuring patterns.
- [ ] Validate tuple and collection destructuring shapes.
- [ ] Enforce mutability and assignment rules.
- [ ] Reject assignment to constants or otherwise non-assignable values.
- [ ] Define and validate `_` discard bindings and patterns.

### Arrays, vectors, tuples, indexing, and ranges

- [ ] Validate array and vector construction syntax semantically.
- [ ] Validate collection element type consistency.
- [ ] Validate compile-time collection lengths.
- [ ] Resolve tuple construction and tuple element types.
- [ ] Type-check array indexing.
- [ ] Type-check vector indexing according to the distinct vector syntax.
- [ ] Validate index types.
- [ ] Validate indexing targets.
- [ ] Type-check ranges and inclusive ranges.
- [ ] Validate range endpoint types and compatibility.
- [ ] Validate range use in `for` loops and other range-consuming contexts.
- [ ] Validate string slicing and range-based access according to the language rules.

### Structs, enums, unions, and members

- [ ] Resolve struct construction by type.
- [ ] Validate positional struct construction.
- [ ] Validate named struct construction.
- [ ] Detect missing struct fields.
- [ ] Detect unknown struct fields.
- [ ] Detect duplicate field initialization.
- [ ] Resolve struct member access.
- [ ] Validate field visibility.
- [ ] Resolve enum variants and variant payloads.
- [ ] Validate enum variant construction.
- [ ] Resolve union types and union member/variant access according to the language design.
- [ ] Detect invalid member access on non-member types.

### `into` implementations and methods

- [ ] Register `into` implementations against their target type.
- [ ] Resolve methods by receiver type and method name.
- [ ] Enforce method visibility.
- [ ] Detect duplicate methods and conflicting signatures within an implementation.
- [ ] Validate method return types and parameters.
- [ ] Validate method receiver/target relationships.
- [ ] Define and enforce method lookup precedence against free functions and members.

### Casts and conversions

- [ ] Validate primitive casts.
- [ ] Validate integer signedness and width conversions.
- [ ] Validate integer/floating-point conversions.
- [ ] Validate boolean, character, and string casts exactly as defined by the language.
- [ ] Validate user-defined and `into`-based conversions, if supported by the language rules.
- [ ] Reject unsupported cast pairs.
- [ ] Resolve the resulting type of every cast expression.
- [ ] Preserve the source span of both the cast expression and target type for diagnostics.

### Pointers and memory semantics

- [ ] Validate address-of expressions.
- [ ] Require address-of operands to be addressable values.
- [ ] Resolve the pointee type of every pointer expression.
- [ ] Validate assignment between pointer types.
- [ ] Enforce non-null pointer rules for `^T`.
- [ ] Enforce optional-pointer rules for `?^T`.
- [ ] Validate `None` only where an optional pointer is expected.
- [ ] Validate pointer dereference rules when dereference syntax/semantics are introduced.
- [ ] Define pointer equality and comparison rules.
- [ ] Reject invalid pointer operations.

### Control flow and reachability

- [ ] Validate `if`, `elif`, and `else` condition types.
- [ ] Type-check conditional expressions and shortened conditional forms.
- [ ] Validate `loop` bodies.
- [ ] Validate `while` condition types.
- [ ] Validate `for` iterator/range sources and loop variable types.
- [ ] Build loop-context information for `break` and `continue`.
- [ ] Resolve labeled `break` and `continue` targets.
- [ ] Reject unknown loop labels.
- [ ] Reject `break` and `continue` outside valid loop contexts.
- [ ] Validate `match` scrutinee types.
- [ ] Validate match arm patterns and arm expressions/statements.
- [ ] Check required match coverage/exhaustiveness rules once defined.
- [ ] Track reachable and unreachable statements where required by the language.
- [ ] Define whether unreachable code is an error, warning, or accepted construct.

### Returns and function-body rules

- [ ] Validate every `return` against the enclosing function return type.
- [ ] Reject returned values from `void` functions.
- [ ] Reject missing values from non-void returns.
- [ ] Determine whether every non-void function path returns a value.
- [ ] Validate implicit/explicit final-expression return rules if supported.
- [ ] Validate `return` inside nested control-flow constructs against the correct enclosing function.

### Function flags and compiler-recognized behavior

- [ ] Resolve every `@flag` against the set of supported flags.
- [ ] Validate flag placement.
- [ ] Validate flag argument requirements where applicable.
- [ ] Enforce semantic effects of `@striped`.
- [ ] Enforce semantic effects of `@lossely`.
- [ ] Reject unknown or incompatible flags.
- [ ] Prevent contradictory or duplicate flag combinations where disallowed.

### Macros and semantic boundary

- [ ] Ensure expanded macro output is semantically analyzed exactly like ordinary source.
- [ ] Preserve useful locations for errors originating from macro arguments.
- [ ] Preserve macro-definition locations for errors originating from macro bodies.
- [ ] Distinguish argument-originated and macro-body-originated diagnostics where useful.
- [ ] Reject semantic constructs produced by expansion that are invalid in their surrounding context.

### Embedded C

- [ ] Validate placement of embedded C blocks.
- [ ] Define the semantic boundary between Astery values and embedded C.
- [ ] Validate any required declarations or annotations around embedded C.
- [ ] Reject embedded C usage in contexts where it cannot be lowered safely.
- [ ] Preserve source locations for embedded C diagnostics.

### Semantic diagnostics and invariants

- [ ] Give every semantic failure a stable error category and precise source location.
- [ ] Include the relevant names and types in type and name diagnostics.
- [ ] Avoid reporting downstream errors caused only by an earlier unresolved declaration when recovery is unnecessary.
- [ ] Keep semantic analysis deterministic for the same source program.
- [ ] Ensure no unresolved symbol remains in a successfully analyzed program.
- [ ] Ensure every successfully analyzed expression has a resolved type.
- [ ] Ensure every successfully analyzed call has a resolved target signature.
- [ ] Ensure every successfully analyzed member access has a resolved declaration.
- [ ] Ensure every successfully analyzed control-flow target resolves to a valid target.
- [ ] Produce a semantic representation that contains all information required by the backend without reparsing source text.

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
