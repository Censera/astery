# Syntax Re-design for 26

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
        if condition then break
    }
    loop {
        if condition then break 'name
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
let hello string = "Hello"[1..3]; // ell

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
type Point i64
let point Point = Point { 3.0, 4.0 };
let point Point = Point { x = 3.0, y = 4.0 };

type Node struct {}
into Node {
    fn Point new() {}
}

let node Node = Node.new();

embed C {
    printf("Hello\n");
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
@name

// from `name(arg)` to `name arg`
// only if it's one param
@striped
fn T name(name T) {}

@lossely
fn T name(name T, ...) {}

@lossely
fn T name(name T, ..., name T, name T, ...) {}

// Macros
macro this() { "this" }

macro square(x) {
    x * x
}

let value = square!(4);
```

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
let = {
    a i32 = 123,
    b i32 = 456,
    t i32 = a
};
```

The bindings are processed in declaration order, so `t` may use `a` because `a` was declared earlier in the same block. A binding may not refer to a later binding unless a separate language feature explicitly introduces forward references.

The `let = { ... }` form uses a block expression as its initializer. The block follows the same result rules described above: its final non-terminated expression supplies the value and type of the block.

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

## BitOP & LogicalOP etc.

```rs
// And
&&

// Or
||

// Xor
^^

// Not
!

// Bit And
:&

// Bit Or
:|

// Bit Xor
:^

// Bit Not
:<

// Shift Right
>>

// Shift Left
<<

// Inc
++

// Dec
--

// Operations
+, -, *, /
+=, -=, *=, /=
==, >=, <=, !=
```

## Union

```rs
// Declaration
type Name = Union::<T, T>;

// Aliases
type Result::<T, T> = Union:<T, T>;
type Optional::<T> = Union:<T, None>;

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
