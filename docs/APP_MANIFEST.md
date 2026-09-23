# Aether Application Manifest

The manifest format is experimental.

A future application manifest is expected to describe at least:

- application identifier;
- human-readable name;
- version;
- executable;
- required capabilities.

Example shape:

```toml
id = "org.aether.calculator"
name = "Calculator"
version = "0.1.0"
executable = "calculator"

capabilities = [
  "display",
  "keyboard"
]
```

This is documentation only. It is not yet an implemented loader format.
