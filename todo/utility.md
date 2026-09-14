# Utility TODO

## Compiler infrastructure

- [ ] Preserve source ownership across every source unit in the module graph.
- [ ] Keep compiler stage boundaries explicit.
- [ ] Keep the public compiler API minimal.
- [ ] Keep source, tokens, AST, semantic IR, backend IR, and emitted-artifact ownership explicit.
- [ ] Keep deterministic ordering for source discovery, modules, declarations, diagnostics, and emitted symbols.
- [ ] Keep options explicit without introducing a configuration framework.
- [ ] Keep missing-file, malformed-input, invalid-option, unsupported-target, and backend failures explicit.

## Diagnostics

- [ ] Define stable error categories for lexer, parser, semantic, backend, CLI, and toolchain failures.
- [ ] Render source file, line, and column consistently.
- [ ] Render useful source snippets without obscuring the primary message.
- [ ] Highlight the smallest useful semantic span.
- [ ] Include declaration/candidate information in overload failures.
- [ ] Include both operand types in type/operator errors.
- [ ] Include expected and actual types in assignment/call/return errors.
- [ ] Include useful imported-module and macro-origin context.
- [ ] Keep diagnostic wording stable enough for tests.
- [ ] Ensure source locations survive every compiler stage.
- [ ] Add warning infrastructure only when a real warning class exists.

## Repository hygiene

- [ ] Keep source layout small and understandable.
- [ ] Remove obsolete modules, dead code, and compatibility layers after migrations.
- [ ] Keep dependencies justified and minimal.
- [ ] Prefer the Rust/JDK standard library where practical.
- [ ] Keep public APIs minimal and explicit.
- [ ] Keep backend wrappers limited to real Astery concepts.
- [ ] Keep terminal output free of decorative formatting that harms scripting.
- [ ] Keep repository commands documented and reproducible.
- [ ] Keep generated artifacts out of source directories unless intentionally required.
- [ ] Remove temporary development files before release.
- [ ] Keep commits focused and descriptive.
- [ ] Avoid CI/tooling that adds maintenance without solving a concrete problem.
- [ ] Keep repository-editing and code-generation helpers deterministic and narrow.

## Dependency and toolchain

- [ ] Pin or constrain LLVM/an-inkwell compatibility.
- [ ] Define how LLVM components are obtained.
- [ ] Define behavior when the expected LLVM version is unavailable.
- [ ] Decide whether LLVM is bundled, externally discovered, or distributed through a supported package.
- [ ] Keep compiler builds reproducible.
- [ ] Document native toolchain assumptions.
- [ ] Ensure release binaries do not depend on undeclared development paths.
- [ ] Test dependency upgrades before changing the minimum toolchain.

## Performance and robustness

- [ ] Establish baseline compile-time measurements before optimizing.
- [ ] Measure lexer/parser/semantic/backend cost separately when needed.
- [ ] Avoid whole-file rescans or reparsing when owned information is sufficient.
- [ ] Bound memory use for large source files and module graphs.
- [ ] Measure generated-code quality before adding optimization passes.
- [ ] Define behavior for very large macros, deeply nested expressions, and large module graphs.
- [ ] Reject pathological inputs cleanly when resource exhaustion cannot be handled safely.
- [ ] Treat source and embedded C as untrusted compiler input.
- [ ] Validate source-discovery and generated-artifact paths.
- [ ] Avoid unsafe fallback behavior when external tools fail.
- [ ] Create and clean temporary files safely.
- [ ] Avoid embedding accidental local paths in deterministic builds.
- [ ] Avoid undefined behavior in compiler/runtime support code.
- [ ] Add regression coverage for fuzzing and malformed-input crashes.
- [ ] Fuzz the lexer and parser after grammar stabilization.

## Experimental features

- [ ] Keep lambda/thunk experiments isolated until the core function/value model is stable.
- [ ] Keep experimental module behavior isolated from the main module system.
- [ ] Keep advanced macro features isolated until basic expansion is stable and tested.
- [ ] Do not expose experimental syntax as stable behavior without updating syntax documentation and tests.

## Release

- [ ] Run the complete test suite from a clean checkout.
- [ ] Run representative end-to-end programs on every supported target.
- [ ] Verify installation and standard-library/runtime discovery.
- [ ] Verify embedded C on every supported target.
- [ ] Verify representative lexer, parser, semantic, backend, and toolchain diagnostics.
- [ ] Verify deterministic compiler output.
- [ ] Check repository cleanliness after release builds.
- [ ] Update version, release metadata, README, and user documentation.
- [ ] Tag releases only after the documented release checks pass.
