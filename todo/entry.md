# Entry TODO

## CLI

- [ ] Redesign the CLI around the actual compiler workflow.
- [ ] Define source-input commands and options.
- [ ] Define check-only and build/compile behavior.
- [ ] Define output-path and target selection.
- [ ] Define optimization/debug/IR/object output options if exposed.
- [ ] Define quiet, verbose, and diagnostic modes without noisy default output.
- [ ] Define help and version output.
- [ ] Define exit codes for usage, source, semantic, backend, and toolchain failures.
- [ ] Keep CLI parsing independent from compiler internals.
- [ ] Match documented success and error output.

## End-to-end entry behavior

- [ ] Compile minimal valid programs through the executable.
- [ ] Test representative programs using each stable language feature.
- [ ] Test multi-module input.
- [ ] Test embedded C input.
- [ ] Test failure exit codes and diagnostic formatting.
- [ ] Test deterministic output across repeated compilations.

## Release entry

- [ ] Compile the compiler from a clean checkout.
- [ ] Verify installation and CLI behavior from a clean environment.
- [ ] Update version and release metadata.
- [ ] Keep README examples buildable.
- [ ] Remove obsolete `v0.x` and `asteri` terminology from user-facing entry points.
