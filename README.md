# Aether Apps

Application repository for Aether OS.

## Purpose

This repository contains native Aether OS applications and the shared application SDK/interfaces used by them.

Aether Apps are kept separate from the Aether OS kernel and core system so applications can evolve independently.

## Repository layout

- `apps/` — native applications
- `sdk/` — application-facing interfaces and ABI documentation
- `libs/` — reusable application libraries
- `docs/` — application development documentation

## Current status

The repository is the initial application workspace. The application ABI and packaging format are not yet frozen.

The first implementation target is a small native Calculator application, followed by Terminal/File Manager and other desktop applications.

## Development rules

1. Do not depend on Linux, Windows, or a hosted runtime.
2. Prefer native Aether interfaces over kernel-internal APIs.
3. Keep applications isolated from kernel implementation details.
4. Do not declare an ABI stable until it is implemented and tested.
5. Every application must document its required Aether OS capabilities.
6. Runtime claims require runtime evidence.

## Relationship to Aether OS

The operating system itself lives in the separate `aether-os` repository.

This repository is for applications and the application-facing SDK only.
