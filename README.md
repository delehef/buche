A fork of [stderrlog](https://github.com/cardoe/stderrlog-rs/), a logger that
aims to provide a simple case of
[env_logger](https://crates.io/crates/env_logger) that just logs to `stderr`
based on verbosity.

## Documentation

For a working example for [StructOpt](https::/crates.io/crates/structopt),
[clap](https://crates.io/crates/clap), and
[docopt](https://crates.io/crates/docopt) please see the
[crate level documentation](https://docs.rs/buche/*/buche/).

For example binaries showing how
[module level logging](https://github.com/delehef/buche/tree/master/examples/large-example) works, please see the `large-example` crate in `examples/`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
buche = "0.6"
```

and this to your main():

```rust
buche::new().verbosity(args.flag_v).quiet(args.flag_q).init().unwrap();
```

where your args struct is defined as:

```rust
struct Args {
    flag_v: usize,
    flag_q: bool,
    ...
}
```
