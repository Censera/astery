# Syntax Re-design for 26

## Overview

This document is the source of truth for Astery's spelling and grammar in v26. `goal.md` states what the language is for, `design.md` describes the compiler's shape, and `plan.md` gives the order of work. This file covers what the syntax is, why it looks the way it does, and how each part is meant to be built.

### Principles

The syntax serves the goals in `goal.md`: a small compiled language whose behavior can be read from the source, with no hidden runtime.

- Types, pointers, and casts have explicit syntax. A pointer that may be `None` is written `?^T`, and an explicit conversion is written `->`.
- Logical and bitwise operators use separate symbol families, so dropping or adding a character can't turn one into the other.
- A block can stand wherever a value is expected. Its last expression, when it has no semicolon, is the value.
- The grammar is decided from tokens and the kind of each name (type, function, value). Parsing never needs a type. Type checking stays in semantic analysis.
- Features the core language doesn't need sit under "Experimental" and don't shape the core grammar.

### Compiler pipeline

Each source unit passes through these stages in order:

```text
source text
  -> lexer                     tokens
  -> return-type normalizer    tokens, `fn T name` rewritten to `fn [T] name`
  -> macro expansion           tokens, `name!(...)` replaced by the macro body
  -> declaration index         the kind of every top-level name
  -> parser                    AST
  -> semantic analysis         resolved names, types, module graph
  -> backend                   LLVM IR
```

The token stages are `tokenize`, `normalize_function_return_tokens`, and `expand_macros`. The declaration index is specified in "Parsing with name kinds". Every stage before the parser works on tokens alone, so declarations that a macro produces are visible to the index.

### Reading this document

The listings under "Main features" show the syntax. The notes after them give the reason for each group and point at the code that implements it. Function names such as `parse_if` and `top_level_item_end` refer to `src/parser`.

## Main features

```rs
// file: main.as
//! T is a placeholder for the language data types

// Module declaration
mod mygame

// Importing
use library
use { mygame, standard, memory, engine }
use math { variable { that } }
use math { function, variable { that } }

// let
let name = value;
let name T = value;
let name, name, name = value;
// Here { ... } belongs to the let declaration syntax
// It means "declare several bindings"
let {
    name = value,
    name = value,
    name = value
};
let {
    name T = value,
    name T = value,
    name T = value
};
let _ = value;

// Constants
const name T = value;
const name, name, name = value;
const {
    name = value,
    name = value,
    name = value
};
const {
    name T = value,
    name T = value,
    name T = value
};

// fn
fn name() {}
fn main() {}
fn name() { return }
fn T name() {
    return 0
}
name();

// Two functions can have the same name as long as they
// have different params or have different return type.
fn name() {}
fn T name() {}
fn T name(name T) {}

// Control Flow
if condition {}
if condition {} elif condition {} else {}
if condition {
    break
}
if condition {
    continue
} else {
    break
}

// Shortened
// <expr-or-stmt> if condition [else <expr-or-stmt>]
statement if condition;
break if condition else continue
let something = value if condition;
let somethingelse = value if condition else value;

// loops
loop {}
loop 'name {
    loop {
        if condition {
            break
        }
    }
    loop {
        if condition {
            break 'name
        }
    }
}
while condition {}
match variable {
    value {},
    value {},
    value {}
}

// for loop
for i in items {}
// range syntax for
for i in 0..10 {}  // exclusive: 0..9
for i in 0..=9 {}  // inclusive: 0..9

// Names, libraries, flags, and builtins
// Identifiers are ordinary names.
// Library functions are ordinary identifiers and calls.
// @flags attach function settings or compiler-recognized behavior to functions.
// A builtin exists only when the language itself requires compiler-level support.

// Primitive values
let this string = "this";
let that string = "that";

// Sub String
let hello string = "Hello"[1..4]; // ell (exclusive range, like `for`)

// Shadowing is allowed

// Enum
pub enum Name {}
pri enum Name {}
enum Name {
    pub name,
    pri name(T),
    name(),
    pub name {
        name T,
        name T
    },
    name {
        name T,
        name T
    }
}

// Structures
pub struct Name {}
pri struct Name {}
struct Name {
    pub name T,
    pri name T,
}

// Member access
let value = Name.name;

// Structure Implementation
into Name {
    pub fn T name() {}
    pri fn T name() {}
}

// Call function
let value = Name.name();

// User's types init using the type keyword
type Point struct { x f64, y f64 }
let point Point = Point { 3.0, 4.0 };
let point Point = Point { x = 3.0, y = 4.0 };

type Node struct {}
into Node {
    fn Node new() {}
}

let node Node = Node.new();

embed C {
    int x = 1;
}

// cast
// variable -> type

let a = true;
let b = a -> i8; // 1
let c = b -> char; // '1'
let d = c -> string; // "1"
let e = d -> char; // '1'
let f = e -> i8; // 1
let g = f -> bool; // true

// pointer
// T is a type placeholder.
let name ^T = &value;

// None pointer
let name ?^T = &value;
let name ?^T = None;

// Function flags for function's settings
@Name

// from `name(arg, arg)` to `name arg, arg`
// works for any number of parameters
@Stripped
fn T name(name T) {}

@Stripped
fn T name(name T, name T) {}

@Stripped
@Loosely
fn T name(name T, ...) {}

@Loosely
fn T name(name T, ...) {}

@Loosely
fn T name(name T, ..., name T, name T, ...) {}

// Macros
macro this() { "this" }

macro square(x) {
    x * x
}

let value = square!(4);
```

### Notes: modules and imports

`mod` names the source unit's root module, and `use` brings names in from other modules. A source unit has one root module and module names are unique within a compilation. `design.md` assigns building the module graph, and rejecting conflicts in it, to semantic analysis.

Import lists can be read without parsing the rest of the file, because `parse_imports` needs only tokens. The driver can read every unit's imports first and then parse each unit with the imported names already indexed. See "Parsing with name kinds".

### Notes: bindings

`let` declares a binding that can be assigned again. `const` declares one that can't. The type follows the name and is optional, so `let name = value;` and `let name T = value;` have the same shape. The brace form declares several bindings at once and lets each carry its own type.

`parse_binding_declaration` accepts either a name list followed by `=` and a value, or a brace list read by `parse_binding_block`. Shadowing is allowed. `let (a, b) = (b, a);` is legal and binds `a` and `b` again with their values exchanged. The earlier bindings are shadowed, not modified.

### Notes: functions and overloading

A function is `fn`, an optional return type, a name, a parameter list, and a body. The return type comes before the name, so one token after `fn` can't say whether it is looking at a type or a name. `normalize_function_return_tokens` settles that before parsing. It takes the first identifier followed by `(` as the name and wraps everything between `fn` and that name in synthetic `[` and `]` tokens. The parser proper only ever sees `fn [T] name(...)`.

Overloads are resolved statically, as `design.md` requires, so two functions can share a name when their parameters or return types differ. The parser accepts every overload. Choosing between them belongs to semantic analysis, and the name index records an overloaded name once, as a function.

### Notes: control flow

`if`, `elif`, `else`, `loop`, `while`, `for`, and `match` take blocks. The shortened forms, `statement if condition;` and `a if condition else b`, exist for guard clauses such as `return x if y == 0;`, where a full block would bury a one-line exit.

Ranges are half-open with `..` and closed with `..=`, in `for` and in slices alike, so `"Hello"[1..4]` is `ell`.

A loop label is written `'name`. The lexer separates it from a character literal by looking ahead: `'a'` closes on a second quote and is a character, and `'a` doesn't and is a label (`apostrophe_token`).

### Notes: casts

`value -> type` converts explicitly. It is a postfix operator, so a chain of conversions reads left to right. The parser collects the target type as tokens up to a terminator (`parse_cast_target_type`), and semantic analysis validates the pair.

Some conversions in the listing change representation instead of only width: `1 -> char` gives `'1'` and `'1' -> i8` gives `1`. Each pair of types has its own conversion rule, which is why semantic analysis validates casts pair by pair.

### Notes: function flags

`@Name` before `fn` attaches a compiler-recognized setting to the function. To the lexer a flag is an `@` token followed by an identifier, and `skip_modifiers` steps over it when the parser splits the file into top-level items.

`@Stripped` lets a call drop its parentheses: `name(a, b)` is written `name a, b`. It works for any number of parameters. `@Loosely` marks a function whose parameter list contains `...`, and it combines with `@Stripped`, which is how a print-style function takes any number of arguments without parentheses.

Both flags change how a call parses, so the name index records each function's flags and parameter count. The count fixes the extent of a stripped call. A function with two parameters takes two comma-separated expressions, so in `g(f a, b)` both belong to `f` when `f` has two parameters, and `b` belongs to `g` when `f` has one. Each argument is a full expression that extends as far right as it can, so `f a + b` is `f(a + b)`. After the name of a stripped function, any token that can begin an expression begins its first argument, `-1` and `&x` included.

For a function with `...`, the fixed parameters come first, and every further comma-separated expression joins the call up to the end of the enclosing expression. `g(print a, b)` gives `print` both arguments, and `g((print a), b)` gives `b` to `g`.

A `(` directly after the name is always a parenthesized call. A stripped function whose overloads differ in parameter count can't be called without parentheses, because the parser can't tell which count applies.

### Notes: macros and embedded C

`macro` definitions and `name!(...)` calls are expanded on tokens before parsing (`expand_macros`). A macro can't inspect types or resolve names, and the parser never sees a macro call. Expansion runs before the declaration index, so a declaration a macro produces is indexed like any other. `expand_macros` receives one token stream, which makes macros visible only inside the unit that declares them.

`embed C { ... }` carries C source across the Astery and C boundary. The body is C, not Astery, so it is read from raw source text with its own brace matching (`parse_embedded_blocks`) and never interpreted as Astery.

## Parsing with name kinds

### What

A `{` after a name can mean three different things:

```rs
Point { x = 3.0, y = 4.0 }   // struct literal, Point is a type
print { a = b } { a = t }    // call with block arguments, print is a function
if flag { ... }              // start of the body, flag is a value
```

The parser tells them apart by the kind of the name in front of the brace. Every name has one kind:

| Kind           | Declared by                                                           | A following `{` means      |
| -------------- | --------------------------------------------------------------------- | -------------------------- |
| Type           | `struct`, `enum`, `type`, the primitive types                         | struct literal             |
| Function       | `fn`                                                                  | call with block arguments  |
| Callable value | a binding initialized with a lambda                                   | call with block arguments  |
| Vector         | a binding or parameter typed `T<...>`, or initialized with `<...>`    | not part of the expression |
| Value          | any other binding, parameter, loop variable, or pattern variable      | not part of the expression |
| Unknown        | not declared anywhere the parser can see                              | not part of the expression |

"Not part of the expression" leaves the brace to the surrounding construct. After `if`, `elif`, `while`, `for ... in`, and `match` it opens the body. Anywhere else it is a parse error.

### Rules

- The kind of a name comes from its declaration alone. The parser never asks for a type. A binding `let f = make();` is a Value even when `make` returns something callable, so `f` is called as `f(x)`.
- Conditions get no exception. In `if check { ... }`, where `check` is a function, the braces are read as its argument and the body is reported missing. A condition that calls a function uses parentheses: `if check() { ... }`.
- A parenthesized call, `name(arg)`, works for every callable and is never ambiguous.
- Local names shadow outer names from their declaration onward, following "Scope" below. After `let print = 1;`, `print` is a Value in that block.
- A name declared with more than one kind is ambiguous. Semantic analysis reports the conflicting declarations, and the parser reports only a brace form used on an ambiguous name.
- Brace-argument calls apply to plain names and to `Type.function` paths where the function is declared in an `into` block for that type. A method called on a value, `value.method`, takes parentheses, because the kind of `value.method` depends on the type of `value`.
- Vector indexing follows the same table. `name<index>` is an index when `name` is a Vector and a comparison otherwise. Inside an index, `>` ends the index, so a comparison there needs parentheses.

### Why

Brace forms are common here: struct literals, block arguments, and block expressions. Rust keeps its grammar simple by refusing struct literals in condition position, which is a rule readers have to learn. Astery gives the parser one more fact, the kind of each name, and reads braces the same way everywhere. The cost is a pass before parsing and a scope stack inside the parser.

The same fact settles `name<index>` against `name < index`, which no token-level rule can.

### Boundaries

The parser learns the kind of a name. It doesn't learn which declaration the name refers to, or its type. Name resolution, overload selection, visibility, and type checking stay in semantic analysis, as `design.md` specifies. The parser's scope stack tracks names and kinds only. It must agree with semantic analysis on where a binding starts and ends and on shadowing, and both follow "Scope" below.

### How

#### Building the index

1. Split each unit's token stream, after macro expansion, into top-level items. `parse_program` already does this: `top_level_item_end` finds where an item ends and `first_declaration_kind` says what it is. Turn that into one function that returns a list of items, each with its kind, token range, and modifiers.
2. Read the declared name from each item.
   - `struct`, `enum`, and `type` give a Type.
   - `fn` gives a Function. The name is the identifier before `(`, which `find_function_name` finds, and the parameter count and flags come from the declaration and its `@` modifiers.
   - `let` and `const` give a Value, or a Vector when the declared type is `T<...>` or the initializer starts with `<`. A comma list, a brace list, or a tuple gives one entry per name.
   - `macro` names go in a separate table, because macros are always called with `!`.
   - `into Name { ... }` declares no name. Its functions are recorded as members of `Name`.
3. Store the result as a `NameIndex`: a map from name to kind, and a map from type name to member functions. Iteration must not depend on hash order, because `design.md` requires deterministic diagnostics. Use ordered maps or sort before iterating.

The index never reports errors. When scanning an item fails, record nothing for it, skip to the next top-level declaration keyword, and let the parser report the real error.

#### Imports

Each source unit gets an index of its own top-level declarations, respecting `pub` and `pri`. Building it needs no other unit, so file order doesn't matter and a cyclic import can't stall this pass. The names visible while parsing a unit are its own index plus the entries that its `use` declarations select from other units' indexes. Semantic analysis still builds the module graph and rejects cycles and conflicts.

Compilation therefore makes two passes over the ordered `Sources`. The first lexes, normalizes, expands, and indexes every unit. The second parses every unit with its visible names.

#### In the parser

`Parser` takes the visible global names and keeps a stack of local scopes. A scope opens at each block, and at function entry where the parameters are added. Names are added as they are declared: `let`, `const`, brace lists, tuple patterns, the variable of a `for`, and match-arm bindings. Each entry carries a kind, decided from the declaration's tokens as in step 2.

`parse_postfix_expression` uses the kinds. After it reads a primary expression that is a plain name or a `Type.function` path, it looks at the next token:

- `{` after a Type: parse struct-literal fields.
- `{` after a Function or Callable value: parse one block expression, then keep parsing block expressions for as long as the next token is `{`. The statement ends at `;`.
- `<` after a Vector: parse an index expression up to `>`.
- An expression start after a Function with `@Stripped`: parse its arguments as "Notes: function flags" describes.
- Anything else: stop, and let the caller decide. `parse_if`, `parse_while`, `parse_for`, and `parse_match` then expect the `{` that opens their body.

When a body is expected and the condition ended in a brace-argument call, the error says so: "`check` is a function, so `{` was read as its argument; write `check()` to call it."

## Blocks and expressions

A block is a sequence of statements with an optional final expression. A block can be used as the body of a function or control-flow construct, and it can also appear where an expression is expected.

The value of a block is determined by its final item:

```rs
{
    let a i32 = 10;
    let b i32 = 20;
    a + b
}
```

The block above is an expression whose type is the type of `a + b`.

A final expression without a semicolon produces the block value. A final expression followed by a semicolon does not produce a value:

```rs
{
    123
}

{
    123;
}
```

The first block has type `i32`. The second block has no value and therefore has the unit/void-like result type defined by the language.

Statements inside the block are evaluated in order. Their values, when they have any, are not implicitly returned by the block. Only the final expression determines the block result.

A block may therefore contain declarations and then return a value built from them:

```rs
{
    let a i32 = 123;
    let b i32 = 456;
    a + b
}
```

The final expression may also be another block, allowing nesting:

```rs
{
    let value = {
        let a i32 = 10;
        a * 2
    };
    value + 1
}
```

Blocks used as statements do not need to produce a value. Blocks used as expressions must obey the normal expression typing rules. A context that requires a value must reject a block whose final expression does not produce one.

### Block grammar

Conceptually, a block follows this form:

```text
block = "{" block-item* "}"
block-item = statement | expression [";"]
```

The parser must preserve whether the final expression ended in a semicolon because that distinction changes whether the block has a resulting value.

A block is not a special declaration form. The same block expression rules apply regardless of where the block is used.

### Scope

A block introduces a lexical scope. Bindings declared inside the block are visible from their declaration onward and are not visible after the block ends.

```rs
let value = {
    let a i32 = 123;
    a + 1
};

// `a` is not visible here.
```

Bindings declared earlier in the same block may be referenced by later statements or expressions:

```rs
{
    let a i32 = 123;
    let b i32 = a + 1;
    b
}
```

### Assignment expressions

Assignment is an expression and therefore may appear anywhere an expression is accepted.

```rs
a = b
```

An assignment evaluates the right-hand side and stores the resulting value into the assignable left-hand side. The assignment expression itself produces the assigned value, subject to the language's assignment typing rules.

Assignments may therefore be used as block results:

```rs
{
    a = b
}
```

and may be used as arguments to a function call:

```rs
print { a = b } { a = t };
```

The braces in this example are block expressions. Each block evaluates its assignment and passes the resulting block value to `print`.

The left-hand side of an assignment must be assignable. At minimum this includes mutable local bindings. Constants and other non-assignable expressions cannot appear on the left-hand side.

Assignment does not declare a new binding. Declaration and assignment remain separate operations:

```rs
let value i32 = 10;
value = 20;
```

### Multi-binding declarations inside blocks

A block may contain ordinary `let` declarations and multi-binding declarations. For example:

```rs
let {
    a i32 = 123,
    b i32 = 456,
    t i32 = a
};
```

The bindings are processed in declaration order, so `t` may use `a` because `a` was declared earlier in the same list. A binding may not refer to a later binding unless a separate language feature explicitly introduces forward references.

The `let { ... }` form is a declaration list, not a block expression. It has no initializer and produces no value. `const { ... }` follows the same rules.

For a block consisting only of declarations, the block does not implicitly return the last declaration. A value is returned only by an actual final expression without a semicolon.

### Assignment and block result examples

```rs
fn main() {
    let a i32 = 123;
    let b i32 = 456;
    let t i32 = a;

    print { a = b } { a = t };

    return 0
}
```

The two blocks passed to `print` each contain one assignment expression. Because neither assignment is terminated with a semicolon inside its block, the assignment result becomes the block result.

Adding a semicolon changes the result:

```rs
print {
    a = b;
};
```

Here the assignment is a statement rather than the value-producing final expression of the block.

### Design notes

A block that ends in an expression without a semicolon yields that value, so any place that wants a value can take a block. That includes call arguments, which is why `print { a = b } { a = t };` needs assignment to be an expression: each block has to end in something with a value.

The semicolon decides between a value and no value. The parser must keep it, which is why `block-item` in the grammar carries an optional `;`.

Scope is lexical and shadowing is allowed. The parser's scope stack in "Parsing with name kinds" follows the same rules.

## Experimental

```rs
// Lambdas and Thunks
|t| t * 2
```

## Types

`T` is a type placeholder and is replaced by a concrete type:

| Type                      | What it is                 |
| ------------------------- | -------------------------- |
| `i8`, `i16`, `i32`, `i64` | Signed integers            |
| `u8`, `u16`, `u32`, `u64` | Unsigned integers           |
| `f32`, `f64`              | Floating point             |
| `bool`                    | Boolean (`true` / `false`) |
| `char`                    | Character                  |
| `string`                  | Heap-allocated string      |
| `^T`                      | Non-null pointer to `T`    |
| `?^T`                     | Optional pointer to `T`   |

### Arrays

```rs
let name T[length];
let name T[] = [value, value, value];
let name T[length] = [value, value, value];
let name T[length, value];

let value = name[index];
let value = name[name[index]];
```

### Vectors

```rs
let name T<length>;
let name T<> = <value, value, value>;
let name T<length> = <value, value, value>;
let name T<length, value>;

let value = name<index>;
let value = name<name>;
let value = name<name<index>>;
```

### Tuples

```rs
let (name, name) T;
let (name T, name T);
let (name, name, name) = (value, value, value);
let (name, name, name) T = (value, value, value);
let (name T, name T) = (value, value);
let (name T, _) = (value, _);
let (_, name T) = (_, value);
let (_, name) T = (_, value);
let (name, _) T = (value, _);
let (_, _) = (value, value);

let (a, b) = (1, 2);
let (a, b) = (b, a);
```

### Notes: types

Pointers are explicit. `^T` is never null, and `?^T` is a pointer that may be `None`, so nullability is part of the type (`design.md`). `&value` takes an address. `^` is a single token (`Caret`), so a pointer to a pointer is written with two carets and no `^^` token exists. `parse_pointer_type` handles pointer types.

Arrays are `T[length]` and vectors are `T<length>`. They are written differently so that a reader can tell them apart, and so can the parser, since `[` or `<` follows the type name in a declaration. Tuples bind by position, and `_` skips a slot.

## BitOP & LogicalOP etc.

```rs
// And
&&

// Or
||

// Xor
~~

// Not
!

// Keyword forms accepted by the lexer for the logical operators
// and, or, xor, not

// Bit And
:&

// Bit Or
:|

// Bit Xor
:^

// Bit Not
:<

// Shift Right
:>

// Shift Left
<:

// Inc
++

// Dec
--

// Operations
+, -, *, /
+=, -=, *=, /=
==, >=, <=, !=
```

### Notes: operators

The logical operators are `&&`, `||`, `~~`, and `!`. The keywords `and`, `or`, `xor`, and `not` spell the same operations. The bitwise operators are the `:` family, `:&`, `:|`, `:^`, and `:<`, with shifts `:>` and `<:`. Two separate families mean that a dropped or doubled character can't turn a logical operation into a bitwise one, the C mistake between `&` and `&&`.

Shifts no longer use `<<` and `>>`. With `>>` gone, nested angle brackets close without splitting a token: `name<name<index>>` lexes as `name < name < index > >`.

## Union

```rs
// Declaration
type Name = Union::<T, T>;

// Aliases
type Result::<T, T> = Union::<T, T>;
type Optional::<T> = Union::<T, None>;

// Construction
// Inferred from value's type
// No explicit tag
let r Result::<i64, string> = 5;
let r Result::<i64, string> = "division by zero";

// Matching by type
// Not by handpicked constructor name
match r {
    i64(value) {
        let _ = value;
    },
    string(message) {
        let _ = message;
    }
}

fn Result::<i64, string> divide(a i64, b i64) {
    return "division by zero" if b == 0;
    return a / b
}
```

### Notes: unions and match

A union holds one of several types. Construction infers the member from the value's type, and `match` selects by type instead of by a constructor name, as in `i64(value) { ... }`. That keeps `Result` and `Optional` as ordinary aliases over `Union::<...>` and not as separate built-in types.

The generic opener `::<` is a single token (`Tetraops`) and is distinct from `:<`, bit not. `type_syntax.rs` and `user_type.rs` parse unions and `type` declarations.

## Outputs

### Success

```
Finished [bin] in X ms
```

```
Checked, and everything is OK.
```

### Warning

```
W [src/file.as][Line][Column] Warning message
```

### Error

```
E [src/file.as][Line][Column] Error message
```

### Notes: diagnostics

Every diagnostic carries a source name, a line, and a column. In the compiler that is `Error::Parse { line, column, message }`, with `with_source` adding the file name. Stages report in source order so that output is the same on every run, as `design.md` requires.
