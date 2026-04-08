# cargo-agents

Cargo subcommand for agentic development workflows. Acts as a thin wrapper around [symposium](https://github.com/symposium-dev/symposium).

## Installation

```
cargo install cargo-agents
```

## Usage

```
cargo agents [args...]
```

All arguments are forwarded directly to `symposium`. If `symposium` is not installed, `cargo-agents` will prompt you to install it automatically using `cargo binstall` (if available) or `cargo install`.

## How it works

`cargo-agents` reserves the `cargo agents` subcommand namespace and delegates to the `symposium` binary:

1. Looks for `symposium` on your `PATH`
2. If found, executes it with all provided arguments
3. If not found, offers to install it for you

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
