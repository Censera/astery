# Astery v26 Syntax

This file is the source of truth for Astery's spelling and grammar in v26. `goal.md` states what the language is for, `design.md` describes the compiler's shape, and `plan.md` gives the order of work.

Every section has the rule, examples, and where it matters, a note for semantic analysis (SeMa) and for the backend. Behavior the earlier draft left undefined is marked `(Qn)` and collected in "Open questions" at the end. Nothing marked `(Qn)` is decided.

## Reading guide

- `T` is a type placeholder, `name` is an identifier, `value` is an expression.
- `// error` on an example line means the compiler rejects it.
- `print` in examples is a library function declared with `@Stripped` and `@Loosely`. It is not part of the language.

### Principles

- Types, pointers, and casts have explicit syntax.
- Logical and bitwise operators use separate symbol families, so a dropped or added character can't turn one into the other.
- A block can stand wherever a value is expected. Its last expression, when it has no semicolon, is the value.
- The parser decides the grammar from tokens and the kind of each name (type, function, value). Parsing never needs a type. Type checking stays in SeMa.
- Features the core doesn't need sit under "Experimental" and don't shape the core grammar.

### Compiler pipeline

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

Every stage before the parser works on tokens alone, so declarations a macro produces are visible to the index.

## 1. Lexical rules

### Tokens that need a note

| Token | Name | Meaning |
| --- | --- | --- |
| `^` | Caret | pointer type (prefix) and dereference (postfix). One token, so a pointer to a pointer is two carets. |
| `&` | | address of (prefix). |
| `&&` | And | logical and. `&&x` is never two address-ofs. Write `& &x`. |
| `~~` | Xor | logical xor. |
| `->` | | cast. |
| `..` `..=` | | half-open and closed range. |
| `::<` | Tetraops | generic opener. Distinct from `:<` (bit not). |
| `:&` `:\|` `:^` `:<` `:>` `<:` | | bitwise family. |

`<<` and `>>` don't exist. Nested angle brackets close without splitting a token: `name<name<index>>` lexes as `name < name < index > >`.

### Comments, literals, terminators

```rs
// line comment
let a = 10;          // integer
let b = 2.5;         // float
let c = 'x';         // char
let d = "text";      // string
let e = true;        // bool
```

- Statements end in `;`.
- `mod` and `use` lines take no `;`.
- A final item in a block may omit `;`, which changes the block's value (see "Blocks").

### Labels and characters

The lexer looks ahead to tell them apart. `'a'` closes on a second quote and is a char. `'a` doesn't and is a loop label.

```rs
let c = 'a';         // char
loop 'a { break 'a } // label
```

### Keywords

`mod use let const fn return if elif else loop while for in break continue match enum struct type into pub pri embed macro and or xor not true false None`

## 2. Modules and imports

`mod` names the source unit's root module. `use` brings names in from other modules. A unit has one root module, and module names are unique within a compilation.

```rs
mod mygame

use library
use { mygame, standard, memory, engine }
use math { variable { that } }
use math { function, variable { that } }
```

- `use name` imports one module.
- `use { a, b, c }` imports several modules.
- `use math { x, y }` imports `x` and `y` from `math`.
- Braces nest one level per path component: `use math { variable { that } }` reaches `that` inside `variable` inside `math` (Q2).

`parse_imports` needs only tokens, so the driver reads every unit's imports first, then parses each unit with the imported names already indexed. SeMa builds the module graph and rejects cycles and conflicts.

Visibility is `pub` or `pri` on items, struct fields, enum variants, and `into` functions. The default when neither is written is undecided (Q1).

## 3. Bindings

`let` declares a binding that can be assigned again. `const` declares one that can't. The type follows the name and is optional.

```rs
let count = 0;               // type inferred
let ratio f64 = 0.5;         // type written
let x, y, z = 0;             // three bindings (Q3)
let _ = compute();           // evaluated, result dropped

const LIMIT u32 = 100;
const A, B = 1;
```

### Brace form

`{ ... }` after `let` or `const` is a declaration list. It has no initializer and no value. Each entry can carry its own type. Entries are processed in order, so an entry may use earlier entries.

```rs
let {
    width  i32 = 80,
    height i32 = 24,
    area   i32 = width * height
};

const {
    W = 80,
    H u32 = 24
};
```

```rs
let { a = b, b = 1 };        // error: b is declared later
```

### Assignment and const

```rs
let n = 1;
n = 2;                       // ok
LIMIT = 5;                   // error: LIMIT is const
```

### Shadowing

Shadowing is allowed. A later `let` of the same name creates a new binding and hides the old one from that point on. The old binding is not modified.

```rs
let n = 1;
let n = n + 1;               // new n, value 2
{
    let n = 10;              // inner n
    n
};
// n is 2 here
```

## 4. Functions

A function is `fn`, an optional return type, a name, a parameter list, and a body. The return type comes before the name.

```rs
fn main() {}
fn hello() { return }
fn i32 five() { return 5 }
fn i32 add(a i32, b i32) { return a + b }
fn ?^i32 maybe(p ^i32) { return p }
fn Result<i64, string> divide(a i64, b i64) {
    return "division by zero" if b == 0;
    return a / b
}
```

`return` with no value leaves the function. `return x` ends the item with `x`. A `return` as the last item of a block may omit `;`.

Parameters are `name T`, separated by commas. Whether a function body returns its final expression without `return` is undecided (Q5).

### Overloading

Two functions can share a name when their parameters or their return type differ. The parser accepts every overload. SeMa picks one, and the name index records an overloaded name once, as a function.

```rs
fn show(x i32) {}
fn show(x f64) {}
fn show(x i32, y i32) {}

fn i32 parse(s string) { return 0 }
fn f64 parse(s string) { return 0.0 }

show(1);                     // first
show(1.5);                   // second
show(1, 2);                  // third
let a i32 = parse("1");      // return type selects the first
let b f64 = parse("1.5");    // return type selects the second
```

```rs
fn show(x i32) {}
fn show(x i32) {}            // error: same name, parameters, and return type
let c = parse("1");          // error: both overloads fit
```

SeMa needs the expected type to choose between overloads that differ only in return type.

### Methods

Functions in an `into` block belong to a type and are called through the type name.

```rs
type Node struct { value i32 }

into Node {
    pub fn Node new() { return Node { value = 0 } }
    pri fn i32 secret() { return 1 }
}

let node Node = Node.new();
```

How a method receives the value it is called on is undecided (Q9).

## 5. Function flags

`@Name` before `fn` attaches a compiler-recognized setting. To the lexer a flag is an `@` token followed by an identifier. Flags stack, one per line.

### @Stripped

`@Stripped` lets a call drop its parentheses. `name(a, b)` is written `name a, b`.

```rs
@Stripped
fn i32 twice(x i32) { return x * 2 }

@Stripped
fn i32 add(a i32, b i32) { return a + b }

let a = twice 4;             // twice(4)
let b = twice 4 + 1;         // twice(4 + 1), the argument extends right
let c = twice -1;            // twice(-1)
let d = twice &x;            // twice(&x)
let e = add 1, 2;            // add(1, 2)
let f = twice(4) + 1;        // parentheses: (twice(4)) + 1
```

The parameter count fixes the extent of a stripped call. Each argument is a full expression that extends as far right as it can. After the name of a stripped function, any token that can begin an expression begins its first argument.

Nesting follows the parameter count of the inner function:

```rs
g(f a, b)     // f has two parameters: g(f(a, b))
g(f a, b)     // f has one parameter:  g(f(a), b)
```

A `(` directly after the name is always a parenthesized call.

```rs
twice (4) + 1                // parenthesized call: twice(4) + 1
```

A stripped function whose overloads differ in parameter count can't be called without parentheses, because the parser can't tell which count applies.

### @Loosely

`@Loosely` marks a function whose parameter list contains `...`. Fixed parameters come first. Every further comma-separated expression joins the call.

```rs
@Loosely
fn print(text string, ...) {}

print("a", 1, 2);            // three arguments

@Stripped
@Loosely
fn log(level i32, ...) {}

log 1, "started", name;      // level = 1, then two more
```

With `@Stripped` the extra arguments run to the end of the enclosing expression:

```rs
g(log 1, a, b)               // log gets all of 1, a, b
g((log 1, a), b)             // log gets 1, a and g gets b
```

Type rules for `...` arguments and lists with several `...` are undecided (Q8).

## 6. Control flow

### if, elif, else

```rs
if n < 0 {
    return -1
} elif n == 0 {
    return 0
} else {
    return 1
}

if done { break }
if skip { continue } else { break }
```

A condition that calls a function uses parentheses. The parser reads `{` after a function name as its argument.

```rs
if check() { ... }           // ok
if check { ... }             // error: check is a function, { was read as its argument
```

### Shortened forms

`<expr-or-stmt> if condition [else <expr-or-stmt>]`. They exist for guard clauses where a full block would bury a one-line exit.

```rs
return 0 if b == 0;
break if done;
continue if skip;
break if found else continue
x = y if ok;
let sign = -1 if n < 0 else 1;
let label string = "neg" if n < 0 else "non-neg";
let maybe = n if n > 0;      // value when the condition is false: Q6
```

### loop, while, for

```rs
loop {
    if done { break }
}

while i < 10 { i += 1 }

for item in items {}
for i in 0..10 {}            // 0, 1, ... 9
for i in 0..=9 {}            // 0, 1, ... 9
for i in 0..n {}            // n is a value, so { opens the body
```

### Labels

`loop 'name` names a loop. `break 'name` and `continue 'name` act on the named loop.

```rs
loop 'outer {
    loop {
        if found { break 'outer }
        if skip { continue }
    }
    loop {
        break 'outer if finished;
    }
}
```

### Ranges and substrings

Ranges are half-open with `..` and closed with `..=`, in `for` and in slices alike.

```rs
let a string = "Hello"[1..4];    // "ell"
let b string = "Hello"[0..=1];   // "He"
```

### match

`match` takes a value and a list of arms. An arm is a pattern and a block, separated by commas.

```rs
match n {
    0 { print "zero" },
    1 { print "one" },
    _ { print "many" }
}
```

The wildcard `_` and exhaustiveness for non-union values are undecided (Q7). Matching a union by member type is in "Unions".

## 7. Blocks, scope, and assignment

A block is `{` items `}`. Each item is a statement or an expression with an optional `;`.

```text
block      = "{" block-item* "}"
block-item = statement | expression [";"]
```

The value of a block is its final item, and only when that item has no semicolon. The parser keeps the semicolon because it decides whether the block has a value.

```rs
let a i32 = {
    let x i32 = 10;
    let y i32 = 20;
    x + y
};                           // a is 30

let b = { 123 };             // i32
let c = { 123; };            // error: the block has no value
```

Blocks nest, and the last item can be another block:

```rs
let value = {
    let inner = {
        let a i32 = 10;
        a * 2
    };
    inner + 1
};                           // 21
```

Statements inside a block are evaluated in order. Their values are not returned.

```rs
let d = {
    let a i32 = 1;
    let b i32 = 2;
};                           // error: declarations only, no final expression
```

A context that needs a value rejects a block without one. A statement position accepts it.

### Scope

A block opens a lexical scope. Bindings are visible from their declaration to the end of the block.

```rs
let value = {
    let a i32 = 123;
    let b i32 = a + 1;       // a is visible
    b
};
// a is not visible here
```

### Assignment is an expression

Assignment evaluates the right side, stores it in the assignable left side, and produces the stored value. It can appear anywhere an expression can. The left side must be assignable: a mutable binding, an element, a field, or a dereference (Q15).

```rs
let a i32 = 123;
let b i32 = 456;
let t i32 = a;

a = b;                       // statement
let z = { a = b };           // block value is the assigned value
```

Declaration and assignment stay separate:

```rs
let value i32 = 10;
value = 20;
value i32 = 20;              // error: assignment doesn't declare
```

### Blocks as call arguments

A block that ends in a value can be an argument. Each `{ ... }` after a function name is one argument.

```rs
fn main() {
    let a i32 = 123;
    let b i32 = 456;
    let t i32 = a;

    print { a = b } { a = t };   // two blocks, two arguments
    print { a = b; };            // the block has no value
    return 0
}
```

In the first `print`, neither assignment ends in `;`, so each block's value is the assigned value. In the second, the `;` turns the assignment into a statement and the block has no value.

## 8. Types

| Type | What it is |
| --- | --- |
| `i8` `i16` `i32` `i64` | signed integers |
| `u8` `u16` `u32` `u64` | unsigned integers |
| `f32` `f64` | floating point |
| `bool` | `true` or `false` |
| `char` | character |
| `string` | heap-allocated string |
| `^T` | non-null pointer to `T` |
| `?^T` | pointer to `T`, or `None` |
| `T[n]` | fixed-length array |
| `T<n>` | growable vector |
| `(A, B)` | tuple |
| `Union<A, B>` | holds one of `A` or `B` |

The type of an unsuffixed integer literal is undecided (Q20).

## 9. Pointers

Two type spellings and two operators cover the whole feature:

| Syntax | Reads as | Where |
| --- | --- | --- |
| `^T` | non-null pointer to `T` | type position |
| `?^T` | pointer to `T`, or `None` | type position |
| `&v` | the address of `v` | expression, prefix |
| `v^` | the value `v` points to | expression, postfix |

```rs
let x i32 = 5;
let p ^i32 = &x;             // p holds the address of x
let y i32 = p^;              // y is 5
```

`^T` is never null. `?^T` can be `None`. A `^T` converts to `?^T`. The reverse is an error.

```rs
let a ^i32 = &x;
let b ?^i32 = &x;            // ok: ^i32 widens to ?^i32
let c ?^i32 = None;          // ok
let d ^i32 = None;           // error: ^i32 is never None
let e ^i32 = c;              // error: c may be None
let f i32 = c^;              // error: c may be None (narrowing: Q15)
```

### Pointer to pointer

Two carets, no `^^` token. Dereference chains the same way.

```rs
let x i32 = 5;
let p ^i32 = &x;
let pp ^^i32 = & &p;         // & & because && is logical and
let y i32 = pp^^;            // 5
```

### Through a pointer

`v^` can be read and, per the assignment rules, written (Q15).

```rs
fn add_one(p ^i32) {
    p^ = p^ + 1;
}

let n i32 = 1;
add_one(&n);                 // n is 2
```

### Address-of needs an addressable operand

```rs
let p ^i32 = &x;             // binding
let q ^i32 = &arr[2];        // element
let r ^f64 = &point.x;       // field
let s ^i32 = &5;             // error: a literal has no address
```

### Pointers and members

Postfix operators apply left to right. `p^.x` dereferences, then reads a field.

```rs
let p ^Point = &pt;
let x = p^.x;
```

### Compiler notes

- SeMa keeps `^T` and `?^T` as distinct types and checks the conversions above.
- The backend lowers both to LLVM `ptr`. Nullability exists only in SeMa.
- `&x` forces `x` into a stack slot (an `alloca`). `v^` is a load and `v^ = e` is a store. The load type comes from the pointee type in SeMa, because LLVM pointers are opaque.
- Returning `&local` from a function leaves a dangling pointer. No rule covers it yet (Q15).

## 10. Arrays

Arrays have a fixed length written in the type.

```rs
let a i32[3];                        // three elements
let b i32[] = [1, 2, 3];             // length from the initializer
let c i32[3] = [1, 2, 3];            // length checked against the initializer
let d i32[8, 0];                     // eight elements, all 0

let first = b[0];
let picked = b[a[1]];                // index with an index
b[0] = 9;                            // assign to an element
```

```rs
let e i32[2] = [1, 2, 3];            // error: three values for length two
```

The contents of `a` with no initializer are undecided (Q4). Slicing arrays with ranges is undecided (Q19).

## 11. Vectors

Vectors are growable and heap-backed. The element type is written before `<...>`.

```rs
let v i32<>  = <1, 2, 3>;            // length from the initializer
let w i32<3> = <1, 2, 3>;            // length checked
let x i32<8, 0>;                     // eight elements, all 0
let y i32<4>;                        // length 4 or capacity 4: Q18

let first = v<0>;                    // index
let mixed = v<w<0>>;                 // index with an index
v<1> = 20;                           // assign to an element

v.push(4);
v.pop();
let n = v.length();
```

`name<index>` is an index when `name` is a vector and a comparison otherwise. The parser decides from the name's kind.

```rs
let i = 3;
let ok = i < 10;                     // i is a value: comparison
let e = v<i + 1>;                    // v is a vector: index
```

Inside an index, `>` ends the index. A comparison there needs parentheses.

```rs
let bad = v<a > b>;                  // the first > ends the index
let fine = v<(a > b)>;
```

### Compiler notes

- SeMa types a vector as `Vector(T)`. `v<i>` needs an integer index and yields `T`. `push(x)` needs `x` convertible to `T`. Literal elements must agree on one `T`.
- `push`, `pop`, and `length` read like ordinary methods but methods come from `into` blocks. SeMa special-cases them or a library declares them.
- The backend emits `{ptr, len, cap}`. `push` reallocates when `len == cap`. Who frees the buffer is undecided.

## 12. Tuples

A tuple groups values by position. `_` skips a slot.

```rs
let (a, b) = (1, 2);
let (a, b, c) = (1, 2, 3);
let (a, b, c) i32 = (1, 2, 3);       // one type for every name
let (a i32, b f64) = (1, 2.5);       // a type per name
let (a i32, _) = (1, 2);             // second slot skipped
let (_, b i32) = (1, 2);
let (_, _) = (1, 2);

let (p, q) i32;                      // declared without values
let (p i32, q f64);
```

Tuple bindings shadow. `let (a, b) = (b, a);` binds new `a` and `b` whose values are the old ones exchanged. The earlier bindings are hidden, not modified.

```rs
let (a, b) = (1, 2);
let (a, b) = (b, a);                 // a is 2, b is 1 from here on
```

## 13. Unions

`Union<A, B, ...>` is an ordinary type that holds one value of one of its member types. `None` is a member type. A union is a value like any other: it can be returned, stored, passed, or produced by a block. `match` is one way to consume it, and it is not the only way to produce it.

### Declaring

```rs
type Number = Union<i64, f64>;
type Result<T, T> = Union<T, T>;     // two type slots
type Optional<T> = Union<T, None>;
```

`Result` and `Optional` are ordinary aliases, so they are not built-in types. The generic opener `::<` is a single token distinct from `:<`. `type_syntax.rs` and `user_type.rs` parse unions and `type` declarations.

### Producing a union

Any position whose expected type is a union accepts a value of a member type. SeMa wraps the value.

```rs
let a Number = 5;                            // i64 member
let b Number = 2.5;                          // f64 member
let r Result<i64, string> = 5;
let e Result<i64, string> = "division by zero";
let o Optional<i64> = None;
let p Optional<i64> = 7;
```

Return:

```rs
fn Result<i64, string> divide(a i64, b i64) {
    return "division by zero" if b == 0;     // string member
    return a / b                             // i64 member
}
```

Block value and branches:

```rs
let r Result<i64, string> = {
    let x i64 = 4;
    x * 2
};

let s Optional<i64> = n if n > 0 else None;  // each branch converts on its own
```

Arguments and fields:

```rs
fn show(r Result<i64, string>) {}
show(5);
show("oops");

struct Reply { pub body Result<i64, string> }
let reply = Reply { body = 5 };
```

Each of these is a conversion site. SeMa passes the expected type down into the expression. Without it, `n if n > 0 else None` has two unrelated branch types and fails.

### Consuming

`match` selects by member type. The arm names the type and binds the value, not a constructor.

```rs
match divide(10, 2) {
    i64(value)      { print "ok", value },
    string(message) { print "error", message }
}

match p {
    i64(value) { print value },
    None       { print "nothing" }      // spelling of the None arm: Q7
}
```

Other ways to consume a union (propagating early, unwrapping) don't exist yet. Each would need its own rule.

### Errors

```rs
type Bad = Union<i64, i64>;                  // error: duplicate member
let x Result<i64, string> = 2.5;             // error: f64 is not a member
let y Union<i64, i32> = 5;                   // ambiguous unless literal typing picks one (Q20)
let z i64 = divide(4, 2);                    // error: a union is not an i64
```

### Compiler notes

- Members are identified by type, so duplicate members must be rejected. `Result<i64, i64>` is an error for the same reason.
- Nested unions such as `Optional<Optional<i64>>` are undecided: flatten, or reject the ambiguity of a bare `None` (Q17).
- The backend assigns each member a tag and lays out `{tag, payload}` with the payload sized and aligned for the largest member. `None` has an empty payload. A conversion site stores the tag and the payload. `match` lowers to a `switch` on the tag.
- If `?^T` is `Optional<^T>`, the tag can be dropped and `None` stored as null, as Rust does for `Option<&T>`. Whether it is the same type is undecided (Q15).

## 14. Structs, enums, and user types

### Structs

```rs
pub struct Name {}
pri struct Name {}

struct Point {
    pub x f64,
    pub y f64,
    pri id i32,
}

type Point struct { x f64, y f64 }
```

The relationship between `struct Name {}` and `type Name struct {}` is undecided (Q10).

Literals are positional or named:

```rs
let a Point = Point { 3.0, 4.0 };
let b Point = Point { x = 3.0, y = 4.0 };
let c = Point { 1.0, 2.0 };
let d = Point { x = 1.0, y = 3.0 };
```

Member access:

```rs
let px = a.x;
a.y = 9.0;
```

### Enums

```rs
pub enum Name {}
pri enum Name {}

enum Shape {
    pub point,                       // no data
    pri circle(f64),                 // one value
    empty(),                         // empty list
    pub rect { w f64, h f64 },       // named fields
    hidden { id i32 }
}
```

Constructing and matching variants is undecided (Q11).

### into blocks

`into Name { ... }` declares no name. Its functions are members of `Name`.

```rs
type Node struct {}

into Node {
    fn Node new() { return Node {} }
    pub fn i32 depth() { return 0 }
}

let node Node = Node.new();
```

A method called on a value takes parentheses, because the kind of `value.method` depends on the type of `value`.

## 15. Casts

`value -> type` converts explicitly. It is a postfix operator, so it binds like the other postfix operators and chains read left to right.

```rs
let a = true;
let b = a -> i8;             // 1
let c = b -> char;           // '1'
let d = c -> string;         // "1"
let e = d -> char;           // '1'
let f = e -> i8;             // 1
let g = f -> bool;           // true

let n = true -> i8 -> char;  // '1'
```

Postfix means the arrow binds tighter than arithmetic. Parenthesize the left side to cast a whole expression.

```rs
let a = x + y -> i64;        // x + (y -> i64)
let b = (x + y) -> i64;      // cast the sum
```

Some conversions change representation, not only width. `1 -> char` is `'1'`, the digit, and `'1' -> i8` is `1`. Each pair of types has its own rule, so SeMa validates casts pair by pair. The parser collects the target type as tokens up to a terminator (`parse_cast_target_type`).

Pairs the draft doesn't list, and `n -> char` for `n` outside 0..9, are undecided (Q12).

## 16. Operators

### Logical

| Symbol | Keyword | Meaning |
| --- | --- | --- |
| `&&` | `and` | and |
| `\|\|` | `or` | or |
| `~~` | `xor` | xor |
| `!` | `not` | not |

```rs
let ok = a > 0 && b > 0;
let ok2 = a > 0 and b > 0;
let either = x || y;
let differ = x ~~ y;
let flip = !done;
let flip2 = not done;
if not done { ... }          // done is a value, so { opens the body
```

### Bitwise

| Symbol | Meaning |
| --- | --- |
| `:&` | and |
| `:\|` | or |
| `:^` | xor |
| `:<` | not |
| `:>` | shift right |
| `<:` | shift left |

```rs
let masked = flags :& 255;
let both = x :| y;
let toggled = x :^ y;
let inverted = :< x;
let up = x <: 3;
let down = x :> 3;
```

The two families never mix. `a && b` is logical and, `a :& b` is bitwise and, and `a & b` is an error.

### Arithmetic, assignment, comparison

```rs
let s = a + b - c * d / e;
n++;                         // increment
n--;                         // decrement
total += 5;
total -= 1;
total *= 2;
total /= 4;
a == b; a != b; a > b; a < b; a >= b; a <= b;
```

Precedence, associativity, and prefix versus postfix `++`/`--` are undecided (Q13). Parenthesize until they are.

## 17. Macros

`macro` definitions and `name!(...)` calls are expanded on tokens before parsing. A macro can't inspect types or resolve names, and the parser never sees a macro call.

```rs
macro this() { "this" }

macro square(x) {
    x * x
}

let a = this!();             // "this"
let b = square!(4);          // 4 * 4
```

Expansion runs before the declaration index, so a declaration a macro produces is indexed like any other. `expand_macros` receives one token stream, so a macro is visible only inside the unit that declares it. Macro names live in a separate table, because macros are always called with `!`.

Token substitution has a hazard. `square!(1 + 2)` becomes `1 + 2 * 1 + 2`, which is 5, unless expansion adds parentheses. Whether it does is undecided (Q14).

## 18. Embedded C

`embed C { ... }` carries C source across the Astery and C boundary.

```rs
embed C {
    int x = 1;
}
```

The body is C. It is read from raw source text with its own brace matching (`parse_embedded_blocks`) and never interpreted as Astery.

## 19. Parsing with name kinds

### What

A `{` after a name can mean three things:

```rs
Point { x = 3.0, y = 4.0 }   // struct literal: Point is a type
print { a = b } { a = t }    // block arguments: print is a function
if flag { ... }              // body: flag is a value
```

The parser tells them apart by the kind of the name in front of the brace. Every name has one kind.

| Kind | Declared by | A following `{` means |
| --- | --- | --- |
| Type | `struct`, `enum`, `type`, primitive types | struct literal |
| Function | `fn` | call with block arguments |
| Callable value | a binding initialized with a lambda | call with block arguments |
| Vector | a binding or parameter typed `T<...>`, or initialized with `<...>` | not part of the expression |
| Value | any other binding, parameter, loop variable, or pattern variable | not part of the expression |
| Unknown | not declared anywhere the parser can see | not part of the expression |

"Not part of the expression" leaves the brace to the surrounding construct. After `if`, `elif`, `while`, `for ... in`, and `match` it opens the body. Anywhere else it is a parse error.

### Rules

- The kind comes from the declaration alone. The parser never asks for a type. `let f = make();` makes `f` a Value even when `make` returns something callable, so call it `f(x)`.
- Conditions get no exception. In `if check { ... }`, where `check` is a function, the braces are its argument and the body is reported missing. Write `if check() { ... }`.
- `name(arg)` works for every callable and is never ambiguous.
- Local names shadow outer names from their declaration onward, following "Scope". After `let print = 1;`, `print` is a Value in that block.
- A name declared with more than one kind is ambiguous. SeMa reports the conflicting declarations, and the parser reports only a brace form used on an ambiguous name.
- Brace arguments work for plain names and for `Type.function` paths where the function is in an `into` block for that type. `value.method` takes parentheses.
- `name<index>` is an index when `name` is a Vector and a comparison otherwise.

```rs
struct Point { x f64, y f64 }
fn check() {}
let flag = true;
let v i32<> = <1, 2, 3>;

let p = Point { x = 1.0, y = 2.0 };   // Type: struct literal
if flag { ... }                       // Value: body
if check() { ... }                    // parenthesized call, then body
let a = v<0>;                         // Vector: index
let b = flag < 3;                     // Value: comparison
```

### Boundaries

The parser learns the kind of a name. It doesn't learn which declaration the name refers to, or its type. Name resolution, overload selection, visibility, and type checking stay in SeMa. The parser's scope stack tracks names and kinds only, and must agree with SeMa on where a binding starts and ends.

### Building the index

1. Split each unit's token stream, after macro expansion, into top-level items: kind, token range, modifiers.
2. Read the declared name from each item. `struct`, `enum`, `type` give a Type. `fn` gives a Function with its parameter count and flags. `let` and `const` give a Value, or a Vector when the type is `T<...>` or the initializer starts with `<`. `macro` goes in a separate table. `into Name { ... }` records members of `Name`.
3. Store a `NameIndex`: name to kind, and type name to member functions. Use ordered maps, because diagnostics must be deterministic.

The index never reports errors. When scanning an item fails, it records nothing, skips to the next top-level keyword, and lets the parser report the real error.

Compilation makes two passes over the ordered sources. The first lexes, normalizes, expands, and indexes every unit. The second parses every unit with its visible names: its own index plus what its `use` lines select from other units.

## 20. Experimental

```rs
// Lambda
|t| t * 2

let double = |t| t * 2;              // Callable value
double { 4 };                        // block argument
```

## 21. Diagnostics

Every diagnostic carries a source name, a line, and a column. In the compiler that is `Error::Parse { line, column, message }`, with `with_source` adding the file name. Stages report in source order so output is the same on every run.

```text
Finished [bin] in X ms
Checked, and everything is OK.
W [src/file.as][Line][Column] Warning message
E [src/file.as][Line][Column] Error message
```

## Open questions

- Q1. Default visibility when neither `pub` nor `pri` is written, for items, fields, variants, and `into` functions.
- Q2. `use math { variable { that } }`: does it bind `that` or `variable.that`?
- Q3. `let x, y, z = 0;`: does each name get a copy of the value?
- Q4. Scalar declarations without an initializer (`let x i32;`), and the contents of `T[n]` and `T<n>` without one.
- Q5. Does a function body, or the block form of `if` and `match`, yield its final expression without `return`?
- Q6. The value of `x if cond;` when `cond` is false and there is no `else`.
- Q7. `match` wildcard `_`, exhaustiveness for non-union values, the arm spelling for `None`, and enum variant patterns.
- Q8. The type of `...` arguments, and how arguments split when a parameter list has several `...`, as in `fn T name(name T, ..., name T, name T, ...)`.
- Q9. How an `into` method receives the value it is called on. No `self` is defined.
- Q10. `struct Name {}` and `type Name struct {}`: same thing, or different?
- Q11. Enum variant construction and matching syntax.
- Q12. Casts between pairs the draft doesn't list, and `n -> char` when `n` is outside 0..9.
- Q13. Operator precedence, associativity, and prefix versus postfix `++` and `--`.
- Q14. Macro expansion: are substituted arguments parenthesized, and do the body's braces appear in the expansion?
- Q15. Pointers: how `?^T` narrows to `^T`, whether `?^T` is `Optional<^T>`, assignability through `v^`, and the lifetime of `&local`. The old draft's `let ^a = 10;` has no meaning under the three pointer rules and was removed from the examples.
- Q16. Who frees a vector's buffer and a `string`'s heap storage.
- Q17. Nested unions: flatten or reject. Where `::<` is used, given that `Union<...>` is the type-position spelling.
- Q18. `T<n>` without values: initial length or capacity. Return types of `pop` and `length`.
- Q19. Range slices on arrays and vectors, and their result type.
- Q20. The default type of an unsuffixed integer or float literal, which decides `let y Union<i64, i32> = 5;`.
