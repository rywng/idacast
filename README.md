# IdaCast

```
    ____     __        ______              __ 
   /  _/____/ /____ _ / ____/____ _ _____ / /_
   / / / __  // __ `// /    / __ `// ___// __/
 _/ / / /_/ // /_/ // /___ / /_/ /(__  )/ /_  
/___/ \__,_/ \__,_/ \____/ \__,_//____/ \__/
```

View splatoon3's battle stages from your terminal.

[![asciicast](https://asciinema.org/a/eUcpsXutWhy30un36Vh0EvHEV.svg)](https://asciinema.org/a/eUcpsXutWhy30un36Vh0EvHEV)

Data is sourced from [splatoon3.ink](https://splatoon3.ink).

## TODO

- Add `lib.rs` export for code reuse

## Development

```bash
# Use direnv and nix-shell to automatically manage dependencies
echo 'use nix\nmkdir $TMPDIR' > .envrc && direnv allow .
```
