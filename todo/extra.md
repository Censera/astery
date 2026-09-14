# Extra TODO

## Casts and pointers

- [ ] Validate primitive casts.
- [ ] Validate integer signedness/width and integer/floating-point conversions.
- [ ] Validate boolean, character, and string casts.
- [ ] Validate user-defined and `into`-based conversions where supported.
- [ ] Reject unsupported cast pairs.
- [ ] Resolve the resulting type of casts.
- [ ] Preserve source spans for cast expressions and target types.
- [ ] Define failure behavior for lossy or unrepresentable conversions.
- [ ] Validate address-of expressions and addressable operands.
- [ ] Resolve pointee types and pointer assignment rules.
- [ ] Enforce `^T`, `?^T`, and `None` rules.
- [ ] Define dereference, pointer comparison, lifetime, allocation, and deallocation behavior.
- [ ] Validate pointer casts only where explicitly permitted.

## Macros

- [ ] Ensure expanded macro output is analyzed exactly like ordinary source.
- [ ] Preserve argument and macro-definition locations in diagnostics.
- [ ] Define deterministic expansion ordering.
- [ ] Define recursion and expansion-depth failure behavior.
- [ ] Prevent malformed expanded token streams from reaching later stages silently.

## Embedded C

- [ ] Validate embedded C placement and surrounding declarations.
- [ ] Define the semantic boundary between Astery values and C.
- [ ] Preserve embedded-C source locations.
- [ ] Define which Astery and C declarations are visible across the boundary.
- [ ] Define multiple-block compilation and linking behavior.
- [ ] Reject embedded C that requires unavailable target features.

## Tests

- [ ] Test primitive and invalid casts.
- [ ] Test pointer and optional-pointer semantics.
- [ ] Test macro diagnostics and expansion failures.
- [ ] Test embedded C semantic restrictions and source locations.
