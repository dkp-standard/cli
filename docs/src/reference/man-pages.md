# Man Pages

Man pages are generated at build time and written to `docs/man/`. One `.1` file is produced per command and subcommand.

## Build

```sh
cargo build -p dkp-cli --features full
```

## Install

```sh
# System-wide
sudo cp docs/man/dkp*.1 /usr/local/share/man/man1/

# Per-user (add to MANPATH)
mkdir -p ~/.local/share/man/man1
cp docs/man/dkp*.1 ~/.local/share/man/man1/
mandb   # or sudo mandb
```

## Read

```sh
man dkp
man dkp-validate
man dkp-search
```
