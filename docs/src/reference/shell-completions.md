# Shell Completions

Completion scripts are generated at build time and written to `docs/completions/`.

## Build

```sh
cargo build -p dkp-cli --features full
```

## Install

### Bash

```sh
# Add to ~/.bashrc or /etc/bash_completion.d/
source docs/completions/dkp.bash
```

### Zsh

```sh
# Copy to a directory in $fpath
cp docs/completions/_dkp ~/.zsh/completions/
# Then reload completions: compinit
```

### Fish

```sh
cp docs/completions/dkp.fish ~/.config/fish/completions/
```

### PowerShell

```powershell
# Add to your $PROFILE
. docs/completions/dkp.ps1
```
