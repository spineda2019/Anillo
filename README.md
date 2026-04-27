# Anillo
The (work in progress) reference implementation for the Anillo language.

## Building
Builds should be easily reproducible. If they are not, that is a bug, and should
ne reported with an issue.

Simple run 

```sh
cargo build
```
to build the compiler in debug mode (see `cargo build -h` for building this
project with different optimization levels).

To run the compiler, use `cargo run` with your specified arguments:

```sh
cargo run -- # Any args to the Anillo compiler go here
```

To see all possible options, run with the `-h` or `--help` flag. The main use
is to pass an anillo file to the compiler as a single argument:

```sh
cargo run -- path/to/file.ani

# You can also directly invoke the compiler, but cargo run rebuilds if it has to
/path/to/anillo path/to/file.ani
```

## Documentation
This project uses the lovely cargo doc feature to auto-generate a full
documentation website using source-code doc comments. To generate the docs and
open them in your browser, simply run:

```sh
cargo doc --open
```

## TODO
* Create homepage to host on github
    * Generate with cargo docs
    * List in Cargo.toml

## Branch organization
Branch names don't really matter, but this repo will try its best at allowing
only passing code to the main branch (enforcing CI to pass before PRs are
merged).

## Handling Cargo.lock
I tend to hate committing lockfiles, but since this is an executable app and
not a lib (and giving those curious an easy way to deterministically rebuild
would be ideal), we can keep this committed.
