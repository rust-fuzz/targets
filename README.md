# targets

A collection of fuzzing targets written in Rust.

## How do I fuzz?

As an example:

```sh
# with AFL
./fuzz-with-afl.sh url_read

# with LibFuzzer
./fuzz-with-libfuzzer.sh url_read

# with Honggfuzz
./fuzz-with-honggfuzz.sh url_read
```

## Contributing

Want to add another fuzz target? It can be for an existing crate or a new one, just open a pull request!

## License

All files in this repository are licensed [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
