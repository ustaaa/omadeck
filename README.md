# OmaDeck

OmaDeck adapts [DeckCraft](https://github.com/storytold/deckcraft) to Omarchy and Linux. This development fork starts at **v0.12.0**, commit `29f0f7772c47aa2bfb74fcbb0c4d3b1503300849`. It is independent of ArtCraft and the Omarchy project.

The presentation engine, file formats, slide rendering, text layout, media codecs, commands and existing ribbon arrangement remain upstream-compatible. Omarchy v3/v4 colors and an optional fontconfig monospace face apply to interface chrome. Slide, chart, notes and print/PDF typography keep their original font paths. Preferences offer Follow Desktop and manual appearance; live font changes wait until editing/input is idle.

Public executables are `omadeck` and `omadeck-cli`; internal Rust package names stay `deckcraft` and `deckcraft-cli`. Desktop ID: `io.github.ustaaa.OmaDeck`. Configuration, logs and recovery live in OmaDeck's own configuration directory. Automatic upstream updates and release publishing are disabled. The installed Omarchy pilot launcher selects native Wayland with `OMADECK_WAYLAND=1` to avoid fractional-scaling blur. Direct binary invocations retain upstream display selection; `env -u OMADECK_WAYLAND -u DECKCRAFT_WAYLAND omadeck` explicitly allows the X11 fallback.

## Build and validate

Use Rust 1.95 or newer, the Linux Wayland/X11, fontconfig, ALSA, GTK/portal and graphics dependencies, and DeckCraft's exact release font input. Its Latin fonts are additional to GridCraft's font pack.

```sh
git clone https://github.com/storytold/craft-fonts ../omadeck-craft-fonts
git -C ../omadeck-craft-fonts checkout --detach 37827adc41fd165f9a9234ac40f94dbad353aa69
export CRAFT_FONTS_DIR="$(cd ../omadeck-craft-fonts && pwd)"
export CRAFT_FONTS_REQUIRED=1
umask 022
cargo xtask ci
cargo build --locked --release -p deckcraft -p deckcraft-cli
packaging/linux/package.sh --skip-build --formats tar
```

Keep `CRAFT_FONTS_DIR` exported for packaging too: it supplies the license files even with `--skip-build`. Preserve a custom `CARGO_TARGET_DIR` in both commands. The user-local pilot installer accepts the generated native tarball:

```sh
packaging/linux/install-pilot.sh /absolute/path/omadeck-0.12.0-linux-aarch64.tar.gz pilot-20261029
# Remove only pilot launcher links; version files are retained:
packaging/linux/install-pilot.sh --uninstall
```

Installation does not alter GridCraft/DeckCraft installations or MIME defaults. The optional loopback control server inherits upstream's protocol and has no authentication; normal pilot launches leave it disabled.

See [HANDOFF](HANDOFF.md) for evidence and limitations and [suite validation](docs/suite-validation.md) for content-equivalence checks. A successful native build is not evidence of full PowerPoint parity. Multi-monitor presentation, portals, accessibility, IME and file drag/drop require their own runtime checks.

## Attribution

Code remains MIT OR Apache-2.0. Original DeckCraft/ArtCraft contributors and notices are preserved in [NOTICE](NOTICE), [ATTRIBUTION](ATTRIBUTION.md), and the About credits. OmaDeck uses original generated launcher artwork. Upstream technical documentation is retained as reference; its release/install identity instructions may describe DeckCraft.
