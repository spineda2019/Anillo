# Anillo
TBD

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
