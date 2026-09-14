# TODO

This is the implementation checklist for v26.8.

The syntax document is authoritative for language spelling and surface behavior. Every unchecked language item below must be validated against `doc/syntaxdesign-26.8.md` before implementation. Do not add compiler architecture merely because a future feature might need it.

## Foundation

- [x] Define the v26 syntax source of truth.
- [x] Establish the compiler library and executable entry point.
- [x] Establish explicit compiler errors.
- [x] Keep compiler stage boundaries explicit.
- [x] Keep the public compiler API limited to the operations users actually need.
- [x] Define the ownership boundary between source, tokens, AST, semantic IR, backend IR, and emitted artifacts.
- [x] Define the compiler input model for one source unit and multiple related source units.
- [x] Define the compilation unit and module graph used by semantic analysis.
- [x] Define deterministic ordering for source discovery, modules, declarations, diagnostics, and emitted symbols.
- [x] Define compiler options without introducing a configuration framework.
- [x] Define target and output settings needed by the native compiler.
- [x] Define failure behavior for missing files, malformed input, invalid compiler options, unsupported targets, and backend failures.

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
- [x] Validate all lexical escape sequences required by the language.
- [x] Validate unterminated strings, characters, comments, and embedded blocks with precise locations.
- [x] Define numeric literal lexical rules and reject malformed forms deterministically.
- [x] Preserve sufficient source information for every token used by diagnostics.
- [x] Keep lexer errors independent from parser and semantic errors.

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
- [x] Validate parser rejection cases for every grammar branch rather than relying only on successful examples.
- [x] Ensure malformed nesting and delimiter errors point at the smallest useful parser location.
- [x] Ensure parser precedence and associativity exactly match the syntax specification.
- [x] Ensure all expression forms have unambiguous parsing when syntax overlaps, especially calls, member access, vectors, ranges, casts, and shortened conditionals.
- [x] Ensure parser recovery does not produce misleading AST structures when recovery is used.
- [x] Keep parser-only syntax details out of semantic structures when they have no later use.

## Parser to semantic boundary

- [x] Accept the `fn T name()` return-type form through the compiler parser boundary.
- [x] Accept the same return-type form for `into` methods.
- [x] Establish one semantic type representation for primitive, user-defined, pointer, array, vector, tuple, and union types.
- [x] Make Semantic a real diagnostic stage instead of only a stage name.
- [x] Represent casts, address-of, and pointer expressions inside the normal semantic expression model.
- [x] Expand macros before semantic analysis while preserving useful source locations.
- [x] Build one complete program/module parse unit for semantic analysis.
- [x] Attach source spans to semantic inputs so type and name errors use their real source locations.
- [ ] Define the semantic representation for type aliases and parameterized union types.
- [ ] Define the semantic representation for visibility, function flags, and compiler-recognized attributes.
- [ ] Define the semantic representation for overload sets and function signatures.
- [ ] Define the semantic representation for collection lengths and compile-time dimensions.
- [ ] Define which parser constructs are erased after semantic lowering and which must survive to backend lowering.
- [ ] Ensure backend lowering never needs to reparse source text.

## Semantic analysis

### Semantic model and source ownership

- [x] Define the semantic program representation produced from the parser program.
- [x] Define source spans for every semantic input that can produce a diagnostic.
- [x] Preserve the original source location of declarations, identifiers, type references, literals, operators, expressions, statements, and macro-expanded input.
- [x] Make semantic errors point at the smallest useful source span instead of a generic file position.
- [x] Define the distinction between parser errors, semantic errors, and backend errors.
- [x] Define which parser constructs are discarded after semantic lowering and which information must remain for diagnostics and code generation.
- [ ] Preserve source ownership for every source unit participating in a module graph.
- [ ] Associate every semantic declaration and reference with its source span.

### Modules, imports, and namespaces

- [x] Register the root module for each source unit.
- [ ] Detect duplicate module, import, and namespace declarations where forbidden.
- [ ] Resolve module paths and imported modules.
- [ ] Detect missing modules and missing imported items.
- [ ] Resolve imported names and grouped imports.
- [ ] Detect ambiguous names introduced by imports.
- [x] Define the namespace rules for functions, bindings, types, enum variants, struct members, and methods.
- [ ] Enforce module visibility across module boundaries.
- [ ] Define module path lookup relative to the input source and configured roots.
- [ ] Define whether duplicate imports are harmless, rejected, or merged according to the language rules.
- [ ] Define the difference between importing a module, importing a name, and importing a grouped name.
- [ ] Define whether imported names are re-exported or remain private to the importing module.
- [ ] Produce deterministic diagnostics for import cycles or mutually dependent modules once module dependencies are supported.
- [ ] Detect module graph cycles when the language or backend cannot support them.

### Symbols, scopes, and name resolution

- [ ] Build symbol tables for module, function, implementation, and local scopes.
- [ ] Define lexical scope creation and destruction rules.
- [ ] Resolve identifiers to exactly one declaration or produce a precise unresolved-name error.
- [ ] Define shadowing behavior and validate legal shadowing.
- [ ] Detect duplicate declarations inside the same scope.
- [ ] Resolve parameters, local bindings, constants, functions, types, enum variants, struct fields, and methods through the correct namespace.
- [ ] Track the declaration associated with each resolved semantic reference.
- [ ] Distinguish value, type, function, field, method, enum-variant, and module namespaces where required by the syntax.
- [ ] Resolve names consistently inside nested scopes and nested control-flow blocks.
- [ ] Ensure lookup order is deterministic when local and imported declarations share names.

### Types

- [ ] Resolve every primitive type.
- [ ] Resolve user-defined types and aliases.
- [ ] Resolve structs, enums, and union types as concrete semantic types.
- [ ] Resolve parameterized union aliases and their concrete type arguments.
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
- [ ] Define layout-relevant type properties needed by the backend.
- [ ] Define compile-time rules for dimensions and lengths used in arrays and vectors.
- [ ] Validate type aliases against the concrete types they name.
- [ ] Reject malformed or unsupported recursive aliases and recursive type expansion.

### Literals and value typing

- [ ] Infer the types of integer, floating-point, boolean, character, string, and `None` literals.
- [ ] Validate literal values against their target types and ranges.
- [ ] Define literal type inference when no explicit type is provided.
- [ ] Reject invalid literal-to-type assignments.
- [ ] Define numeric literal conversion rules without silently changing value meaning.
- [ ] Validate character literals against the language character representation.
- [ ] Validate string literals against the language string representation.

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
- [ ] Define operator behavior for pointers and optional pointers exactly where the language permits it.
- [ ] Define operator behavior for collection and string expressions where supported.
- [ ] Validate short-circuit behavior for logical operators.
- [ ] Validate evaluation order wherever the language guarantees one.

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
- [ ] Define exact overload identity rules for same-name functions differing by parameter types.
- [ ] Define exact overload identity rules for same-name functions differing by return type.
- [ ] Prevent an overload set from containing signatures that cannot be distinguished by the language call rules.
- [ ] Resolve `@striped` calls according to the documented one-parameter call form.
- [ ] Resolve `@lossely` calls and validate variadic argument placement and types.
- [ ] Preserve the selected declaration/signature for backend lowering.

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
- [ ] Define whether binding initialization is guaranteed before the binding can be referenced.
- [ ] Reject self-referential initialization where invalid.

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
- [ ] Define result types for array/vector/tuple/string indexing and slicing.
- [ ] Define out-of-bounds behavior for runtime indexing and slicing.
- [ ] Validate statically known invalid indices where the language makes them knowable at compile time.

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
- [ ] Validate enum variant visibility.
- [ ] Define the semantic representation of unit, tuple, and field enum variants.
- [ ] Define the representation and matching rules for union values constructed by inferred value type.
- [ ] Validate union match arms by contained type.
- [ ] Detect duplicate or overlapping union match arms.
- [ ] Define exhaustiveness rules for enum and union matches.

### `into` implementations and methods

- [ ] Register `into` implementations against their target type.
- [ ] Resolve methods by receiver type and method name.
- [ ] Enforce method visibility.
- [ ] Detect duplicate methods and conflicting signatures within an implementation.
- [ ] Validate method return types and parameters.
- [ ] Validate method receiver/target relationships.
- [ ] Define and enforce method lookup precedence against free functions and members.
- [ ] Ensure `into` blocks cannot target invalid or conflicting types.
- [ ] Associate each method with its implementation and target type for backend lowering.

### Casts and conversions

- [ ] Validate primitive casts.
- [ ] Validate integer signedness and width conversions.
- [ ] Validate integer/floating-point conversions.
- [ ] Validate boolean, character, and string casts exactly as defined by the language.
- [ ] Validate user-defined and `into`-based conversions, if supported by the language rules.
- [ ] Reject unsupported cast pairs.
- [ ] Resolve the resulting type of every cast expression.
- [ ] Preserve the source span of both the cast expression and target type for diagnostics.
- [ ] Define whether casts are compile-time, runtime, or both depending on the source/target pair.
- [ ] Define failure behavior for conversions that can lose information or cannot represent the source value.

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
- [ ] Define pointer lifetime assumptions that affect code generation and embedded C interaction.
- [ ] Define allocation and deallocation responsibilities for values represented through pointers.
- [ ] Ensure optional pointer representation has one consistent backend representation.
- [ ] Validate pointer casts only where explicitly permitted.

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
- [ ] Define control-flow joins needed for binding initialization and type state.
- [ ] Validate shortened conditional forms in both statement and expression contexts.

### Returns and function-body rules

- [ ] Validate every `return` against the enclosing function return type.
- [ ] Reject returned values from `void` functions.
- [ ] Reject missing values from non-void returns.
- [ ] Determine whether every non-void function path returns a value.
- [ ] Validate implicit/explicit final-expression return rules if supported.
- [ ] Validate `return` inside nested control-flow constructs against the correct enclosing function.
- [ ] Validate function entry parameters before body analysis.
- [ ] Define behavior for functions that can terminate without explicit return.

### Function flags and compiler-recognized behavior

- [ ] Resolve every `@flag` against the set of supported flags.
- [ ] Validate flag placement.
- [ ] Validate flag argument requirements where applicable.
- [ ] Enforce semantic effects of `@striped`.
- [ ] Enforce semantic effects of `@lossely`.
- [ ] Reject unknown or incompatible flags.
- [ ] Prevent contradictory or duplicate flag combinations where disallowed.
- [ ] Ensure flags are preserved into backend decisions only where their documented semantics require it.

### Macros and semantic boundary

- [ ] Ensure expanded macro output is semantically analyzed exactly like ordinary source.
- [ ] Preserve useful locations for errors originating from macro arguments.
- [ ] Preserve macro-definition locations for errors originating from macro bodies.
- [ ] Distinguish argument-originated and macro-body-originated diagnostics where useful.
- [ ] Reject semantic constructs produced by expansion that are invalid in their surrounding context.
- [ ] Define macro expansion ordering and deterministic expansion behavior.
- [ ] Define recursion and expansion-depth failure behavior.
- [ ] Prevent macro expansion from producing malformed token streams that reach later stages silently.

### Embedded C

- [ ] Validate placement of embedded C blocks.
- [ ] Define the semantic boundary between Astery values and embedded C.
- [ ] Validate any required declarations or annotations around embedded C.
- [ ] Reject embedded C usage in contexts where it cannot be lowered safely.
- [ ] Preserve source locations for embedded C diagnostics.
- [ ] Define how embedded C is passed to the target compiler/toolchain.
- [ ] Define which Astery declarations become visible to embedded C and which C declarations become visible to Astery.
- [ ] Define compilation and linking behavior for multiple embedded C blocks.
- [ ] Reject embedded C constructs that require unavailable target features.

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
- [ ] Ensure diagnostics from multiple source units identify the correct source file.
- [ ] Define whether semantic analysis stops at the first error or accumulates independent errors.
- [ ] Keep error ordering deterministic.

## Backend

### Backend boundary

- [ ] Define the Astery to `an-inkwell` boundary.
- [ ] Confirm every required LLVM operation exists in the `an-inkwell` API before depending on it.
- [ ] Keep LLVM handles private to backend implementation types.
- [ ] Keep Astery semantic types independent from raw LLVM types.
- [ ] Define backend error conversion and preserve the originating source context where possible.
- [ ] Define target-dependent backend capabilities without leaking target-specific details into semantic analysis.

### LLVM module and symbol lowering

- [ ] Create one LLVM module for the compilation unit or define the exact multi-module strategy.
- [ ] Assign deterministic LLVM symbol names to Astery modules, functions, globals, and generated helpers.
- [ ] Detect LLVM symbol collisions before emission when possible.
- [ ] Lower visibility into the appropriate LLVM/linkage model.
- [ ] Declare external functions required by the standard library and embedded C.
- [ ] Lower function flags that have a backend effect.
- [ ] Verify generated LLVM IR before native emission.

### Values and primitive types

- [ ] Lower constants and primitive values.
- [ ] Lower integer types.
- [ ] Lower floating-point types.
- [ ] Lower booleans.
- [ ] Lower characters.
- [ ] Lower string representation.
- [ ] Lower `None` for optional pointers.
- [ ] Define canonical backend representations for each primitive type.
- [ ] Define ABI behavior for every type passed or returned by functions.

### Functions and calls

- [ ] Lower function declarations and definitions.
- [ ] Lower function parameters.
- [ ] Lower function return values.
- [ ] Lower local function calls.
- [ ] Lower imported/module-qualified function calls.
- [ ] Lower overloaded call targets after semantic resolution.
- [ ] Lower method calls after semantic resolution.
- [ ] Lower flagged and variadic calls.
- [ ] Lower external/runtime calls.
- [ ] Preserve the semantic signature when constructing LLVM function types.

### Expressions and operations

- [ ] Lower arithmetic and logical operations.
- [ ] Lower comparisons.
- [ ] Lower bitwise operations.
- [ ] Lower shifts.
- [ ] Lower increment and decrement.
- [ ] Lower compound assignments.
- [ ] Lower ranges.
- [ ] Lower casts.
- [ ] Lower address-of and pointer operations.
- [ ] Lower string slicing and indexing.
- [ ] Lower collection indexing.
- [ ] Lower conditional expressions.
- [ ] Lower all expression forms accepted by the semantic stage.

### Locals and memory

- [ ] Lower local bindings.
- [ ] Lower constants.
- [ ] Lower mutable storage where required.
- [ ] Lower loads and stores.
- [ ] Lower allocation where the language requires runtime allocation.
- [ ] Lower deallocation where the language requires explicit or generated cleanup.
- [ ] Lower pointer values and nullability representation.
- [ ] Preserve alignment and target data-layout requirements.
- [ ] Define temporary-value lifetime behavior for composite values and strings.

### Composite values

- [ ] Lower arrays and array construction.
- [ ] Lower vectors and vector construction.
- [ ] Lower tuples and tuple access.
- [ ] Lower structs and field access.
- [ ] Lower enums and variant payloads.
- [ ] Lower unions and their runtime representation.
- [ ] Lower named and positional aggregate construction.
- [ ] Define layout and alignment for every composite type.
- [ ] Define ABI passing/return behavior for composite values.

### Control flow

- [ ] Lower basic blocks.
- [ ] Lower conditional branches.
- [ ] Lower `if`, `elif`, and `else` control flow.
- [ ] Lower `loop`.
- [ ] Lower `while`.
- [ ] Lower `for` over ranges and supported iterable sources.
- [ ] Lower `break` and `continue`.
- [ ] Lower labeled loop exits.
- [ ] Lower `match` branching.
- [ ] Lower return paths.
- [ ] Ensure every backend basic block is structurally valid.

### Runtime safety and target behavior

- [ ] Define runtime behavior for out-of-bounds access.
- [ ] Define runtime behavior for invalid casts that cannot be proven safe statically.
- [ ] Define runtime behavior for division-by-zero where applicable.
- [ ] Define runtime behavior for null optional-pointer use.
- [ ] Define runtime behavior for allocation failure.
- [ ] Define whether checks are emitted, delegated to the runtime, or rejected statically.
- [ ] Keep generated failure behavior deterministic and documented.

### Embedded C backend integration

- [ ] Lower embedded C source into the native compilation pipeline.
- [ ] Define temporary/source file handling for embedded C when external toolchains require it.
- [ ] Define include, compile, and link argument propagation.
- [ ] Preserve useful mapping between generated native diagnostics and Astery source locations.
- [ ] Ensure embedded C cannot silently bypass required compiler/linker configuration.

### Optimization and verification

- [ ] Define the minimum optimization level used by the compiler.
- [ ] Keep optimization choices separate from language semantics.
- [ ] Verify generated LLVM IR structurally before target emission.
- [ ] Add optimization only after measuring generated-code problems.
- [ ] Ensure debug/unoptimized builds remain usable for development.

## Runtime and standard library

### Runtime boundary

- [ ] Define exactly which facilities require a runtime component.
- [ ] Keep the runtime smaller than the language library surface.
- [ ] Define the ABI between generated Astery code and runtime functions.
- [ ] Define target-specific runtime responsibilities without hiding ordinary language operations behind implicit behavior.

### Core operations

- [ ] Implement `print`.
- [ ] Implement `eprint`.
- [ ] Implement `read` with the documented zero-argument source syntax and string result.
- [ ] Implement `sizeof`.
- [ ] Implement `length`.
- [ ] Implement `format`.
- [ ] Define the accepted argument and return types of every core operation.
- [ ] Define formatting rules for every supported printable value.
- [ ] Define input encoding and termination behavior for `read`.
- [ ] Define error behavior for standard-library I/O failures.

### Memory and strings

- [ ] Define the runtime representation of heap-allocated strings.
- [ ] Define string allocation, resizing, slicing, and destruction behavior.
- [ ] Define ownership/lifetime rules for strings returned by runtime functions.
- [ ] Ensure `format` and `read` use the same agreed string representation as the compiler/backend.
- [ ] Ensure `length` has defined behavior for every supported collection/string type.

## Native output and toolchain

- [ ] Define supported host targets for v26.8.
- [ ] Define target triples and default-target behavior.
- [ ] Define object-file emission.
- [ ] Define executable emission.
- [ ] Define static/shared library emission only if the language requires it.
- [ ] Define linker discovery and invocation.
- [ ] Define C compiler/toolchain discovery for embedded C.
- [ ] Define standard-library/runtime object discovery.
- [ ] Define target-specific linker arguments.
- [ ] Define output naming and output-directory behavior.
- [ ] Define deterministic artifact naming.
- [ ] Report toolchain failures with the invoked stage and useful context.
- [ ] Make missing LLVM/toolchain components fail explicitly rather than falling back silently.
- [ ] Verify produced artifacts on each supported target.

## CLI

- [ ] Redesign the CLI around the actual compiler workflow instead of the current single-positional-file prototype.
- [ ] Define source-input commands and options.
- [ ] Define check-only behavior.
- [ ] Define build/compile behavior.
- [ ] Define output-path behavior.
- [ ] Define target selection.
- [ ] Define optimization selection if exposed.
- [ ] Define debug information selection if exposed.
- [ ] Define LLVM IR/object output modes if exposed.
- [ ] Define verbose/diagnostic modes without noisy default output.
- [ ] Define help output.
- [ ] Define version output.
- [ ] Define exit codes for success, usage errors, source errors, semantic errors, backend errors, and toolchain failures.
- [ ] Keep CLI parsing independent from compiler internals.
- [ ] Ensure CLI error messages follow the documented output style.
- [ ] Match successful output to the syntax specification's documented success messages.

## Diagnostics

- [ ] Define stable error categories for lexer, parser, semantic, backend, CLI, and toolchain failures.
- [ ] Render source-file, line, and column consistently.
- [ ] Render source snippets where useful without obscuring the primary message.
- [ ] Highlight the smallest useful span for semantic errors.
- [ ] Include declaration and candidate information in overload failures.
- [ ] Include both operand types in operator/type errors.
- [ ] Include expected and actual types in assignment/call/return errors.
- [ ] Include available names for unresolved identifiers where the cost and noise are justified.
- [ ] Include imported module/name context in import failures.
- [ ] Include the source origin of macro diagnostics.
- [ ] Keep diagnostic wording stable enough for tests.
- [ ] Ensure errors never silently lose their original source location between stages.
- [ ] Implement warning infrastructure only when there is an actual warning class to report.

## Testing and conformance

### Lexer tests

- [ ] Test every keyword.
- [ ] Test every operator and punctuation token.
- [ ] Test every literal form.
- [ ] Test valid escape sequences.
- [ ] Test malformed literals and unterminated constructs.
- [ ] Test source locations for representative tokens.

### Parser tests

- [ ] Test every documented declaration form.
- [ ] Test every documented expression form.
- [ ] Test every control-flow form.
- [ ] Test arrays, vectors, tuples, structs, enums, unions, pointers, casts, macros, imports, and embedded C.
- [ ] Test overload declarations.
- [ ] Test visibility declarations.
- [ ] Test function flags.
- [ ] Test every known parser rejection case.
- [ ] Test precedence and associativity with mixed expressions.

### Semantic tests

- [ ] Test successful name resolution.
- [ ] Test unresolved names.
- [ ] Test duplicate declarations.
- [ ] Test legal and illegal shadowing.
- [ ] Test imports and grouped imports.
- [ ] Test visibility across modules.
- [ ] Test primitive type resolution.
- [ ] Test user-defined types and aliases.
- [ ] Test arrays, vectors, tuples, structs, enums, unions, and pointers.
- [ ] Test literal typing and literal range errors.
- [ ] Test operator typing.
- [ ] Test calls and overload resolution.
- [ ] Test return checking and missing returns.
- [ ] Test assignment and mutability.
- [ ] Test control-flow targets and match coverage.
- [ ] Test function flags.
- [ ] Test macro semantic behavior and source locations.
- [ ] Test embedded C semantic restrictions.
- [ ] Test deterministic diagnostics and error ordering.

### Backend tests

- [ ] Test LLVM lowering for every supported primitive type.
- [ ] Test LLVM lowering for functions and calls.
- [ ] Test arithmetic, logical, comparison, cast, and pointer operations.
- [ ] Test control flow and loop lowering.
- [ ] Test arrays, vectors, tuples, structs, enums, and unions.
- [ ] Test optional pointers and `None`.
- [ ] Test runtime/standard-library calls.
- [ ] Test embedded C integration.
- [ ] Verify generated LLVM IR for representative programs.
- [ ] Verify that invalid semantic programs never reach backend lowering.

### End-to-end tests

- [ ] Compile minimal valid programs.
- [ ] Compile representative programs using each language feature.
- [ ] Execute produced binaries for supported targets.
- [ ] Test standard-library input and output.
- [ ] Test multi-module programs.
- [ ] Test embedded C programs.
- [ ] Test failure exit codes.
- [ ] Test diagnostic formatting from the CLI.
- [ ] Test deterministic output for repeated compilations.
- [ ] Add regression tests for every fixed compiler bug.

### Syntax conformance

- [ ] Compare implemented surface against every non-experimental section of `doc/syntaxdesign-26.8.md`.
- [ ] Mark any deliberately deferred syntax explicitly in documentation rather than leaving silent gaps.
- [ ] Ensure examples in the syntax document are either valid programs or clearly marked as schematic snippets.
- [ ] Keep TODO completion aligned with the syntax source of truth.

## Documentation

- [ ] Keep `doc/syntaxdesign-26.8.md` authoritative and synchronized with implementation decisions.
- [ ] Update `doc/design.md` when compiler ownership or stage boundaries materially change.
- [ ] Update `doc/plan.md` when implementation order changes for a concrete reason.
- [ ] Update this TODO when a feature is completed, intentionally deferred, removed, or redefined.
- [ ] Document supported targets.
- [ ] Document compiler installation and invocation.
- [ ] Document CLI commands and options.
- [ ] Document standard-library operations.
- [ ] Document module/import behavior.
- [ ] Document type, pointer, cast, overload, visibility, and control-flow rules that are not obvious from examples.
- [ ] Document embedded C requirements and limitations.
- [ ] Document diagnostics and exit codes.
- [ ] Keep README examples buildable as the language becomes executable.
- [ ] Remove obsolete v0.x/asteri terminology from user-facing documentation.

## Developer tooling and repository hygiene

- [ ] Keep source layout small and understandable.
- [ ] Remove obsolete modules, dead code, and compatibility layers after migrations.
- [ ] Keep dependencies justified and minimal.
- [ ] Prefer the JDK/Rust standard library for functionality that does not need an external dependency.
- [ ] Keep public APIs minimal and explicit.
- [ ] Keep `final`-by-default Rust design through ownership and visibility rather than unnecessary abstraction layers.
- [ ] Keep backend wrappers limited to real Astery concepts.
- [ ] Keep diagnostic output free of decorative formatting that harms scripting or terminal use.
- [ ] Keep repository commands documented and reproducible.
- [ ] Keep generated artifacts out of source directories unless intentionally required.
- [ ] Remove temporary development files before release.
- [ ] Keep commits focused and descriptive.
- [ ] Do not add CI or tooling solely for appearance when local compiler checks are sufficient.
- [ ] Add formatting, linting, or CI automation only when it solves an actual maintenance problem.
- [ ] Keep any code-generation or repository-editing helper tools deterministic and narrow.

## Dependency and toolchain management

- [ ] Pin or constrain the LLVM/an-inkwell compatibility required by the compiler.
- [ ] Define how the required LLVM development/runtime components are obtained.
- [ ] Define behavior when the expected LLVM version is unavailable.
- [ ] Decide whether LLVM is bundled, externally discovered, or distributed through a supported toolchain package.
- [ ] Keep compiler builds reproducible on supported development environments.
- [ ] Document the exact native toolchain assumptions.
- [ ] Verify that release binaries do not accidentally depend on undeclared development-only paths.
- [ ] Test dependency upgrades before changing the minimum supported toolchain.

## Performance and resource behavior

- [ ] Establish baseline compile-time measurements before optimizing.
- [ ] Measure lexer/parser/semantic/backend cost separately when performance becomes a problem.
- [ ] Avoid whole-file rescans or reparsing when existing owned information is sufficient.
- [ ] Keep memory use bounded for large source files and module graphs.
- [ ] Measure generated-code quality before adding optimization passes.
- [ ] Define behavior for very large macros, deeply nested expressions, and large module graphs.
- [ ] Reject pathological compiler inputs cleanly where resource exhaustion cannot be handled safely.

## Security and robustness

- [ ] Treat source files and embedded C as untrusted compiler input.
- [ ] Avoid command construction that allows embedded source or paths to alter tool invocation unexpectedly.
- [ ] Validate paths used for source discovery and generated artifacts.
- [ ] Avoid unsafe fallback behavior when external tools fail.
- [ ] Ensure temporary files are created and cleaned up safely.
- [ ] Ensure deterministic builds do not embed accidental local paths unless debug information explicitly requires them.
- [ ] Avoid undefined behavior in compiler and runtime support code.
- [ ] Add regression coverage for crashes discovered during fuzzing or malformed-input testing.
- [ ] Fuzz the lexer and parser after the grammar stabilizes.

## Experimental features

These do not block the first complete v26 implementation unless the main language surface begins depending on them.

- [ ] Evaluate lambda/thunk syntax only after the core function/value model is stable.
- [ ] Evaluate any remaining experimental module behavior not required by the main module system.
- [ ] Evaluate additional macro capabilities only after basic macro expansion is stable and tested.
- [ ] Keep experimental code isolated from the core compiler until it has a concrete implementation requirement.
- [ ] Do not expose experimental syntax as stable language behavior without updating the syntax source of truth and tests.

## Release readiness

- [ ] Compile the compiler itself from a clean checkout.
- [ ] Run the complete automated test suite.
- [ ] Run representative end-to-end programs on every supported target.
- [ ] Verify installation and CLI behavior from a clean environment.
- [ ] Verify the standard library/runtime is included and discoverable.
- [ ] Verify embedded C support on every supported target.
- [ ] Verify diagnostics for representative lexer, parser, semantic, backend, and toolchain failures.
- [ ] Verify deterministic compiler output for repeated builds.
- [ ] Check repository cleanliness after a release build.
- [ ] Update version numbers and release metadata.
- [ ] Update README and user documentation.
- [ ] Tag the release only after the documented release checks pass.
