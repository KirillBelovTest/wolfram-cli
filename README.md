# `wolfram-cli`

#### [CLI Documentation](./docs/CommandLineHelp.md) | [*Changelog*](./docs/CHANGELOG.md) | [Contributing](#contributing)

## About

An unofficial Wolfram command-line interface.

## Usage

See [**Command Line Help**](./docs/CommandLineHelp.md).

## Installing `wolfram-cli`

This project is a development prototype, and must be build from source manually.

To install the `wolfram-cli` command-line tool, first clone this repository:

```shell
$ git clone https://github.com/ConnorGray/wolfram-cli
```

Next, install the `ConnorGray/WolframCLI` paclet locally by executing:

```shell
$ ./wolfram-cli/scripts/install-paclet.wls
```

Finally, install the `wolfram-cli` executable by invoking
[`cargo`](https://doc.rust-lang.org/cargo/):

```shell
$ cargo install --path ./wolfram-cli
```

Verify the installation by executing:

```shell
$ wolfram-cli
```

Which should open an interactive REPL interface.

## Contributing

See [**Development.md**](./docs/Development.md) for instructions on how to perform
common development tasks.

*See [Maintenance.md](./docs/Maintenance.md) for instructions on how to maintain
this project.*