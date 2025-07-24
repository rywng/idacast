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

## TODO

- Add `lib.rs` export for code reuse

## Development

```bash
# Use direnv and nix-shell to automatically manage dependencies
echo 'use nix\nmkdir $TMPDIR' > .envrc && direnv allow .
```

## Credits

This software is licensed under `MIT OR Apache-2.0`.

Data is sourced from [splatoon3.ink](https://splatoon3.ink).

### Compliance with `splatoon3.ink`'s API usage:

- Credit: This software currently uses the following endpoints:
  - Schedule data: <https://splatoon3.ink/data/schedules.json>
  - Translation data: <https://splatoon3.ink/data/locale/en-US.json>, depending on user's language preference.
- Caching: Caching is not implemented (yet). Currently, data are fetched only at program startup, and will be fetched again every 4 hours. User can also manually fetch data by pressing `r`. (e7683ca50fa737488eb8b8fc7676dfd5a43a0403)
- User Agent: useragent is set to `idacast/<version>` (aa736564fe8adbace6f3679b0ac67c37e0c348c8)
- Free of Charge: This software is licensed under `MIT OR Apache-2.0`. (b216bf2876c3b87c0e998aad586bd4c5342aaa66)
