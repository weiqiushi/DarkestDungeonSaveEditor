# Rust

This is a reimplementation of the save decoder/encoder in the Rust programming language.
Mostly experimental, so it won't replace the main editor written in Java. However, Rust
has a few benefits:

* Performance: When compiling with `--release`, the generated code is much faster than
  the Java version.
* WASM: Rust can natively target WebAssembly, so there is a bare-bones browser version
  available at [https://robojumper.github.io/DarkestDungeonSaveEditor/](https://robojumper.github.io/DarkestDungeonSaveEditor/)

## Command-line usage

From this directory, build the command-line tool with `cargo build --release -p ddsaveedit`.

```sh
# Decode a binary save to readable JSON.
target/release/ddsaveedit decode persist.roster.json roster.decoded.json

# Encode edited JSON back to a binary save.
target/release/ddsaveedit encode roster.decoded.json persist.roster.edited.json
```

Both commands require an input path and an output path. Existing output files are
overwritten after the input has been successfully parsed. Failures are reported to
stderr with a nonzero exit status. Use `ddsaveedit --help` to display usage.
