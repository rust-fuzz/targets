# targets

A collection of fuzzing targets written in Rust.

## How do I fuzz?

This repository contains a small CLI tool to help you run our fuzzers.
You can run it with `cargo run` (just like any other Rust tool).
Here are some examples:

- `cargo run list-targets` gives you a list of all fuzz targets
- `cargo run target pulldown_cmark_read` runs the `pulldown_cmark_read` target with the default fuzzer
- `cargo run target pulldown_cmark_read --fuzzer libfuzzer` runs the `pulldown_cmark_read` target with `libfuzzer`
- `cargo run continuously` runs all targets (you can overwrite timeout per target and change the fuzzer)

For a complete list of available options, run `cargo run -- help`.

## Contributing

Want to add another fuzz target? It can be for an existing crate or a new one, just open a pull request!

## License

All files in this repository are licensed [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
