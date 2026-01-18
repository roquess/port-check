# port-check

**port-check** is a fast and flexible CLI tool to inspect TCP ports and display which processes are using them.

It supports both human-readable output and structured formats such as JSON, YAML, TOML, and XML, making it suitable for scripting, automation, and debugging.

## Features

- Check whether a TCP port is free or in use
- Display detailed process information:
  - process name
  - PID
  - user
  - command
  - start time and uptime (when available)
- Multiple output formats:
  - Human-readable (default)
  - JSON
  - YAML
  - TOML
  - XML
- Consistent structured output powered by `loki_weave`
- Cross-platform friendly (Windows, Linux, macOS)

## Usage

```bash
pc <port> [--format <format>]
```

## Output Formats

When using structured formats, the output represents either:
- a single process if the port is used by one process
- a list of processes if multiple bindings are detected
- a status object if the port is free

This makes the output easy to consume programmatically.

## Motivation

`port-check` was designed as a modern alternative to platform-specific tools like `lsof`, `netstat`, or `ss`, with a focus on:
- clean output
- script-friendly formats
- predictable behavior

## License

MIT

