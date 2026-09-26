# Backend TODO

## Backend boundary

- [ ] Define the Astry to `an-inkwell` boundary.
- [ ] Confirm every required LLVM operation exists in the `an-inkwell` API before depending on it.
- [ ] Keep LLVM handles private to backend implementation types.
- [ ] Keep Astry semantic types independent from raw LLVM types.
- [ ] Define backend error conversion and preserve source context where possible.
- [ ] Define target-dependent backend capabilities without leaking target details into semantic analysis.

## LLVM module and symbols

- [ ] Create the LLVM module strategy for the compilation unit.
- [ ] Assign deterministic LLVM symbol names.
- [ ] Detect LLVM symbol collisions before emission where possible.
- [ ] Lower visibility and backend-relevant function flags.
- [ ] Declare external/runtime functions.
- [ ] Verify generated LLVM IR before native emission.

## Types and values

- [ ] Lower constants and primitive values.
- [ ] Lower integer, float, boolean, character, string, and optional-pointer representations.
- [ ] Define canonical backend representations and ABI behavior.

## Functions and calls

- [ ] Lower function declarations, parameters, returns, and calls.
- [ ] Lower module-qualified and external/runtime calls.
- [ ] Lower overloaded targets after semantic resolution.
- [ ] Lower method calls after semantic resolution.
- [ ] Lower flagged and variadic calls.
- [ ] Preserve semantic signatures when constructing LLVM function types.

## Expressions and memory

- [ ] Lower arithmetic, logic, comparisons, bitwise operations, shifts, increments, and compound assignments.
- [ ] Lower ranges, casts, address-of, pointer operations, indexing, slicing, and conditional expressions.
- [ ] Lower local bindings, constants, loads, stores, allocation, deallocation, and composite temporary lifetimes.
- [ ] Lower assignment expressions, which produce the assigned value.
- [ ] Lower block expressions, including final-expression values and the unit result.

## Composite values

- [ ] Lower arrays, vectors, tuples, structs, enums, unions, and aggregate construction.
- [ ] Define layout, alignment, and ABI behavior for composite values.

## Control flow

- [ ] Lower basic blocks and branches.
- [ ] Lower `if`, `elif`, `else`, `loop`, `while`, `for`, `break`, `continue`, labels, `match`, and returns.
- [ ] Ensure every backend basic block is structurally valid.

## Runtime and target behavior

- [ ] Define runtime behavior for bounds errors, invalid casts, division by zero, null optional-pointer use, and allocation failure.
- [ ] Decide which runtime checks are emitted, delegated, or rejected statically.
- [ ] Keep generated failure behavior deterministic.

## Embedded C integration

- [ ] Lower embedded C into the native pipeline.
- [ ] Define temporary/source file handling, include paths, compile flags, and link arguments.
- [ ] Preserve useful mapping between native diagnostics and Astry sources.

## Optimization and verification

- [ ] Define the minimum optimization level.
- [ ] Keep optimization separate from language semantics.
- [ ] Add optimization only after measuring a real code-generation problem.

## Runtime and standard library

- [ ] Define the runtime boundary and generated-code ABI.
- [ ] Implement `print`, `eprint`, `read`, `sizeof`, `length`, and `format`.
- [ ] Define printable types and formatting rules.
- [ ] Define the string representation, allocation, resizing, slicing, destruction, and ownership rules.

## Native output and toolchain

- [ ] Define supported host targets and target triples.
- [ ] Implement object and executable emission.
- [ ] Define linker and C-toolchain discovery.
- [ ] Define runtime/standard-library object discovery.
- [ ] Define output naming and artifact determinism.
- [ ] Report toolchain failures with useful stage context.
- [ ] Fail explicitly when required LLVM/toolchain components are unavailable.
- [ ] Verify produced artifacts on every supported target.

## Tests

- [ ] Test LLVM lowering for every supported primitive type.
- [ ] Test functions, calls, operations, pointers, control flow, collections, composites, runtime calls, and embedded C.
- [ ] Verify generated LLVM IR for representative programs.
- [ ] Verify invalid semantic programs never reach backend lowering.
