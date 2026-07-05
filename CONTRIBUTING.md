# Contributing to dkp

## Building and testing

This is a Rust workspace. Standard cargo workflow:

```sh
cargo build
cargo test
cargo fmt --check
cargo clippy
```

CI runs formatting, lint, test, and doc-build checks on every PR (see `.github/workflows/ci.yml`).

## Developer Certificate of Origin (DCO)

All contributions must be signed off under the [Developer Certificate of Origin](https://developercertificate.org/). This is a lightweight alternative to a CLA: by signing off, you're asserting that you have the right to submit the contribution under this project's license (MIT OR Apache-2.0).

Sign off by adding `-s` to your commit:

```sh
git commit -s -m "your commit message"
```

This appends a `Signed-off-by: Your Name <your.email@example.com>` trailer to the commit message, using the name/email from your git config.

If you forgot to sign off an existing commit:

```sh
git commit --amend -s
```

Pull requests are checked automatically by the DCO GitHub App; a PR with unsigned commits will fail the check until fixed.

## License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as the rest of the project (see `LICENSE-MIT` and `LICENSE-APACHE`).
