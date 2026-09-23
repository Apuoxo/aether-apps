# Calculator implementation notes

## First executable milestone

The first milestone is a native Aether userspace calculator with:

1. a process loaded through the existing ELF loader;
2. keyboard input delivered through an Aether userspace interface;
3. drawing through an Aether userspace display/GUI interface;
4. arithmetic performed entirely in the app;
5. no direct access to kernel internals.

## Current boundary

The current Aether kernel has an ELF loader and process table, but the inspected source does not yet expose a documented, tested application syscall/GUI ABI. Therefore this repository does not guess one.

The arithmetic engine is implemented now. The native executable adapter will be connected only after the kernel-side contract exists.


## CI ELF
The Calculator is built as a freestanding x86_64 ELF (`x86_64-unknown-none`) by GitHub Actions. The resulting ELF is the packaged application image consumed by the Aether OS build; the kernel does not reimplement the calculator logic.
