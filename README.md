# zoom-sync

Cross-platform, open source utility to sync time, weather, and system info on Zoom65 v3 screen modules.

### Third Party Services

The following free third-party services are used to fetch some information:

- Weather forcasting: [open-meteo](https://open-meteo.com)
- Geolocation (for weather): [ipinfo.io](https://ipinfo.io)

## Usage

> Note: On nixos, you must use the flake for nvidia gpu temp to work

```
nix run
# or
cargo run
```

## TODO

- [ ] System tray menu
- [ ] System daemon
- [ ] Crates.io release
- [ ] AUR release
- [ ] Nixpkgs release
- [ ] Windows release
