# Design

Astery is designed as a small compiled language with concrete concepts and explicit ownership.

## Language shape

The language surface is defined by a few core concepts:

```text
Module
Function
Type
Value
Statement
Expression
Struct
Enum
```

The syntax document is authoritative for spelling and grammar. This document describes implementation shape rather than replacing the syntax specification.

## Compiler shape

The compiler is organized as a direct pipeline:

```text
Source
  -> Lexer
  -> Parser
  -> AST
  -> Semantic analysis
  -> Backend
  -> Output
```

A stage should expose the information required by the next stage and nothing more.

## Foundation contract

`Source` owns one source unit's name and text. It is immutable after construction. `Sources` owns the ordered set of related source units and rejects duplicate source names.

A compilation is therefore either one `Source` through `Compiler::compile` or an explicit `Sources` collection through `Compiler::compile_with_options`. Source order is the order supplied by the caller. The compiler must not depend on filesystem enumeration order.

The semantic stage receives parsed source units in that deterministic order. It is responsible for constructing the module graph from each source unit's `mod` declaration and imports. A source unit has one root module. Module names are unique within a compilation unit, and later semantic work must reject conflicts rather than choose one arbitrarily.

The ownership boundary is:

```text
Source / Sources
    owned by compiler input
        |
        v
Tokens
    owned by lexer result
        |
        v
AST / Program
    owned by parser result
        |
        v
SemanticProgram / module graph / resolved symbols and types
    owned by semantic analysis
        |
        v
Backend IR objects
    owned by backend and LLVM context
        |
        v
Emitted output
    owned by the output operation / caller
```

Later stages must not re-read source files to recover information that an earlier stage already owns. Source text may remain available for diagnostics, but semantic and backend work must consume structured data rather than reparsing source text.

The public compiler surface consists of source input, compiler options, and compilation entry points. Parser helper operations remain crate-internal. The lexer/parser data types may remain publicly reachable because they are language representation types, but they are not the primary compiler control API.

Compiler options are deliberately concrete rather than framework-driven. v26 currently defines `Target::Native` and `Output::{Executable,Object}`. Options are validated before compilation. Adding another option requires a concrete compiler need.

The compiler has explicit failure classes. Missing or unreadable files use `Error::Io`. Invalid compiler configuration uses `Error::InvalidOptions`. Targets that the backend cannot support use `Error::UnsupportedTarget`. Lexical, parser, semantic, and backend failures retain their respective stage. No failure is silently recovered from.

Diagnostics and emitted symbols must be deterministic. Within a compilation unit, source units, module declarations, declarations discovered from them, diagnostics, and emitted symbols are processed in stable source/declaration order unless a later backend rule explicitly requires another documented order.

## Ownership

Compiler data should use Rust ownership and lifetimes directly.

Source text, tokens, AST nodes, semantic information, and backend objects should have explicit ownership. Shared mutable global state is not part of the core design.

## Frontend

The lexer converts source bytes into tokens.

The parser converts tokens into the language AST. The AST should represent the language as defined rather than mirror parser implementation details.

Semantic analysis resolves names, types, visibility, overloads, casts, and other rules that cannot be established by parsing alone.

## Backend

The backend lowers semantic program structures into LLVM IR through `an-inkwell`.

Astery should not reproduce LLVM's object hierarchy. Backend types should correspond to real compiler concepts and remain as small as possible.

The backend boundary should keep LLVM handles private. Raw LLVM operations belong in `an-inkwell`.

## Language mapping

The v26 language types map to LLVM only where code generation needs a representation.

Primitive types map directly where LLVM has a corresponding representation. Composite language types are represented according to the operations Astery needs for construction, access, calls, and returns.

Pointers remain explicit. Optional pointers represent the language's `None` state rather than introducing hidden nullable behavior.

## Functions

Functions are concrete language objects with a name, parameters, return type, visibility, flags, and body.

Overloads are resolved statically. Runtime dispatch, reflection, and plugin-based function lookup are not part of the core design.

## Structures and enums

Structs contain typed fields and may have implementations declared with `into`.

Enums represent a finite set of named variants. Variant payloads are part of the enum definition.

These are language concepts, not a general object system imposed over every value.

## Runtime and standard library

Runtime behavior should be explicit. Standard-library operations such as printing and input are ordinary callable facilities from the compiler's perspective.

There is no hidden runtime required merely to make basic language constructs work.

## Experimental features

Modules, macros, lambdas, and thunks remain outside the core implementation until the main language requires them.

Adding an experimental feature must not complicate the core compiler architecture in advance.
