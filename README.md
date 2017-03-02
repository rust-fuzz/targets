# targets

A collection of fuzzing targets written in Rust.

## How do I fuzz?

As an example:

```sh
# Specify RUSTFLAGS so the target crate is compiled with sanitization
export RUSTFLAGS="-Cpasses=sancov -Cllvm-args=-sanitizer-coverage-level=3 -Zsanitizer=address -Cpanic=abort"

# Change directories into the crate we want to fuzz
cd mp4parse

# Start fuzzing using the 'read_mp4' fuzz target
cargo run --bin read_mp4
```

## Contributing

Want to add another fuzz target? It can be for an existing crate or a new one, just open a pull request!

## License

All files in this repository are licensed [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
