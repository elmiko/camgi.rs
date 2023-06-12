# Contributing to camgi.rs

This project uses the `devel` branch as the primary point of development.

All pull requests are expected to branch from `devel` and be proposed as a merge back into
the `devel` branch.

Contributors should add tests for new features, where appropriate, and should run the `make test`
and `make fmt` commands before committing to ensure the best experience with the continuous
integration workflows.

When editing the html module, run this command for live render:

```ShellSession
cargo run --example html-designer
```
