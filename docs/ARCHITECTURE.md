# Aether Apps Architecture

## Separation

Aether OS provides the kernel, drivers, system services, filesystem and desktop environment.

Aether Apps provides user-facing programs and the interfaces required to build them.

The application repository must not silently become a second kernel tree.

## Application model

The intended model is a native x86_64 Aether application running in userspace.

Each application should have:

- a manifest;
- an entry point;
- declared capabilities;
- application-owned state;
- explicit dependencies on SDK interfaces.

## Capabilities

Applications should request only the capabilities they need.

Examples:

- `display`
- `keyboard`
- `mouse`
- `filesystem.read`
- `filesystem.write`
- `process`
- `clock`

The exact capability ABI will be defined from the interfaces actually implemented by Aether OS.

## ABI policy

The ABI is currently experimental.

Do not invent compatibility guarantees before the corresponding Aether OS interfaces exist and have been tested on real hardware.

## Packaging

Application packaging will be introduced after the executable/ABI contract is established.
