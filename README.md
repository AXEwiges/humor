# Humor

Humor is a practical command line tool that supports hierarchical reading of YAML configuration files to execute script instructions.

Humor's idea is to provide developers with a simple tool to manage commonly used commands.

## Roadmap

- [x] Hierarchical configuration
- [x] Execute commands

## Example

Humor can read the default configuration file from ~/.humors, or start reading commands from humor.yaml in the current directory. Humor supports hierarchical reading to meet the call requirements of instructions for different domains, and supports short queries to find the instructions you need.

Example humor.yaml:
```yaml
import:
  - /somepath/test.yaml
  - /somepath/dev.yaml

commands:
  rust:
    prelude:
      check: cargo fmt && cargo check
      build: cargo build
```

Example test.yaml
```yaml
commands:
  python:
    test:
      testall: python -m unittest testall.py
      testmodule: python -m unittest testmodule.py
```

Example dev.yaml
```yaml
commands:
  python:
    dev:
      build: python -m build --wheel
      install: pip install ./dist/somepackage-py3-0.0.1.wheel
```