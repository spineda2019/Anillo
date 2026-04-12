# Anillo
TBD

## TODO
* Create homepage to host on github
    * Generate with cargo docs
    * List in Cargo.toml

## Handling Cargo.lock
I tend to hate committing lockfiles, but since this is an executable app and
not a lib (and giving those curious an easy way to deterministically rebuild
would be ideal), we can keep this committed.
