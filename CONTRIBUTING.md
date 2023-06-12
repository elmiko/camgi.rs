# Contributing to camgi.rs

This project uses the `devel` branch as the primary point of development.

All pull requests are expected to branch from `devel` and be proposed as a merge back into
the `devel` branch.

Contributors should add tests for new features, where appropriate, and should run the `make test`
and `make fmt` commands before committing to ensure the best experience with the continuous
integration workflows.

## HTML Designer

As a convenience, when editing the html module you can invoke a live renderer which will process
the sample test data and create an html page based on that input. It will also run a live server
with auto reload to make the circuit of changing the look and feel more smooth.

To engage the HTML designer, run the following command from the root of the project:

```ShellSession
make html-designer
```
