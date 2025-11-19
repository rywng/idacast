```
    ____     __        ______              __
   /  _/____/ /____ _ / ____/____ _ _____ / /_
   / / / __  // __ `// /    / __ `// ___// __/
 _/ / / /_/ // /_/ // /___ / /_/ /(__  )/ /_
/___/ \__,_/ \__,_/ \____/ \__,_//____/ \__/
```

Simple program to view splatoon3's battle stages from your terminal. It supports different languages [^locales].

> This program is under active development, and API is subject to change.

## Usage

Install with cargo:

```bash
cargo install idacast
```

To see the usage:

```bash
idacast --help
```

### Keybinds

- `r`: Refresh manually
- `q`: Quit
- `j` / `k` / `Mouse Wheel`: Scroll
- `CTRL-L` / `RMB`: Reset scroll
- `TAB` / `S-TAB` / `h` / `l`: Change views

## Screenshots

| Challenges                                                                                                                          | Regular Battle                                                                                                                      |
| ----------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| <img width="1280" height="800" alt="image" src="https://github.com/user-attachments/assets/3cef07a7-fd5e-4451-b624-3cac26f37a47" /> | <img width="1280" height="800" alt="image" src="https://github.com/user-attachments/assets/00bc5926-459d-4412-bd44-99d72e1a026b" /> |

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
- Caching: Caching is implemented, and the program by default fetch data every 4 hours.
- User Agent: `user_agent` is set to `idacast/<version>`.
- Free of Charge: This software is licensed under `MIT OR Apache-2.0`.

[^locales]: <https://github.com/misenhower/splatoon3.ink/wiki/Data-Access#locales>
