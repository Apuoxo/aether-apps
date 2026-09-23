# Aether Calculator

Native Aether OS calculator application.

## Status

Initial implementation stage. The calculator core is written as a pure no_std Rust module so its arithmetic is independent of the Aether kernel and GUI.

The executable/UI adapter is intentionally separate until the real Aether userspace syscall and GUI contract is frozen and runtime-tested.

## Planned operations

- addition
- subtraction
- multiplication
- division
- clear/reset
- decimal integer input

Floating point and advanced scientific functions are deliberately not part of this first stage.

## Layout

- src/lib.rs — calculator state and arithmetic engine.
- src/main.rs — future native Aether executable entry point; not yet wired to an implemented syscall ABI.
