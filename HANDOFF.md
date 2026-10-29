# OmaDeck handoff — 2026-10-29

## Baseline and scope

The user explicitly authorized this independent DeckCraft adaptation, direct fork/commit/push and an OmarchyMax user-local pilot; the workspace's usual Nomura handoff gate is waived for this work only. Source baseline is official stable **v0.12.0**, `29f0f7772c47aa2bfb74fcbb0c4d3b1503300849`. Repository: https://github.com/ustaaa/omadeck, branch `codex/omarchy-ui`; `upstream` retains https://github.com/storytold/deckcraft.

Engine, model, import/export, document fonts, renderers and media source directories are unchanged. Internal Cargo package names remain upstream names; binaries are `omadeck` / `omadeck-cli`, desktop ID `io.github.ustaaa.OmaDeck`. Host config/log/recovery is isolated under OmaDeck. The opt-in loopback control protocol is unchanged and **has no authentication**. Normal pilot launch does not enable it. Upstream release checking, self-update and publishing workflows are disabled; PR/main/manual quality checks remain active.

## Appearance and typography

Read-only bounded Omarchy v3/v4 palette reader, desktop portal fallback, last-good palette retention, fontconfig monospace chrome, Follow Desktop/manual preference, original family-style screen icon. The reader polls from a background worker and never executes theme files. Transient missing/malformed palettes keep the last good answer; desktop portal state remains cached for manual-to-system transitions. Font updates wait until input/editor/IME activity is idle; palette changes can apply immediately.

The named `omadeck-ui` / chrome bold font families may receive the desktop font. Default proportional and `omadeck-document-bold` retain original font chains. Slide raster text, chart/table rendering, notes body, notes master, notes print preview and presenter notes preserve document typography. A separate UI-only role prevents changing saved/exported typography. Existing ribbon layout is retained.

## Reproducible environment

Both Linux hosts use official Deck font pack **37827adc41fd165f9a9234ac40f94dbad353aa69**, at `~/Development/Codex/omadeck-craft-fonts`. This differs deliberately from the Grid font pin: Grid's older pack lacks Deck's required Inter Latin manifest. Export `CRAFT_FONTS_DIR` and `CRAFT_FONTS_REQUIRED=1` for **both build and packaging**, including `--skip-build`, so font licenses accompany binary redistribution.

ARM OmarchyMax `100.71.47.82`: source `~/Development/omadeck`, target `~/.cache/omadeck-build/arm64`, build at most `-j2`. x86_64 UrazHalo `100.100.203.27`: same source path, target `~/.cache/omadeck-build/amd64`, `-j4`. Halo had runtime ALSA but no development package and sudo was unavailable; official distro dev package was extracted user-locally to `~/.cache/omadeck-build/alsa-dev/root`. Set `PKG_CONFIG_PATH=$HOME/.cache/omadeck-build/alsa-dev/root/usr/lib/x86_64-linux-gnu/pkgconfig`; no host system package/config was changed. Build with umask 022. Host Rust is 1.99.1; GitHub CI pins upstream's 1.95.0 (not yet remotely executed).

## Executed checks

- Full UI library: **425 passed, 0 failed, 4 ignored**. Includes physical Text and Japanese IME preedit/commit pending-font regressions, one-step undo and deferred application after edit exit.
- Workspace excluding PDF crate: **2108 passed, 0 failed, 6 ignored**, across 58 test/doctest harnesses. UI results above are a subset, not additive.
- Strict workspace/all-target clippy: passed. Asset attribution: 58 assets passed. Dependency layers: 28 crates passed. Pinned PDF dependency boundary: passed. Wasm check: all 23 wasm-safe crates passed (three target-only dead-code warnings). Remaining PDF suite excluding the baseline-reproduced test passed.
- Full workspace run has one **verified upstream failure**, `deckcraft_pdf::tests::unembeddable_font_returns_error_instead_of_a_pdf_missing_text`, `crates/pdf/src/tests.rs:232`: expects one added font face, receives 12 with this environment's resolved font. Same failure reproduced from pristine stable checkout `~/Development/Codex/omadeck-baseline-validation`, log `~/.cache/omadeck-build/baseline-pdf.log`. No PDF/engine source was changed to hide it. Do not describe aggregate `cargo xtask ci` as green.
- The language-dialog test had a host dictionary suffix (`Français (France)  (dictionary)`); selector now accepts either label while preserving click and applied-language assertions. Two standalone presenter harnesses now bootstrap named UI fonts like the app; notes assertions explicitly check document families. These are test setup/role updates, not production engine fixes.
- Differential corpus: **2/2 passed** against official upstream release CLI, complete command catalog equal, full semantic/shape/notes/table observations and four rendered slide PNG hashes identical. No reported lost slides, damaged media or missing fonts. Six validator fault-injection tests passed. Evidence `dist/validation/corpus-amd64-r2`, methodology `docs/suite-validation.md`.
- Actual OmarchyMax native Wayland window (`OMADECK_WAYLAND=1`) rendered gold/dark chrome with original slide/notes typography. Screenshot `dist/validation/native/01-actual-omarchy.png` was visually reviewed by both implementation and root agent.
- Isolated native theme smoke: live dark→light; malformed/deleted palette last-good retention; manual dark survives updates; Follow Desktop resumes light; document inspection unchanged by all appearance transitions; sample saved and reopened. Evidence `dist/validation/native/live-theme.json`, `02-isolated-light.png`. It used a private XDG state directory, not the user's active theme. Port 18801 test process exited.

## Limits and next work

This is an installed-development-pilot scope, not a GTK/Qt port. Direct binary launches retain upstream default XWayland selection; the installed Omarchy pilot desktop entry explicitly sets `OMADECK_WAYLAND=1`, following the user-reported Grid XWayland blur at 1.6 monitor scaling. Native Wayland was exercised on OmarchyMax. `env -u OMADECK_WAYLAND -u DECKCRAFT_WAYLAND omadeck` retains a manual X11 fallback. Broad GNOME, scaling, accessibility/screen-reader, native file-picker, clipboard, Wayland external file-drop and physical IME device coverage remain acceptance work. CLI/kittest success is not a real desktop input-method certification. Flatpak sandbox theme-file access and non-Linux release packaging are not validated. Mac/Windows historical packaging remains reference-only, inactive release workflows use `.upstream` suffix. No external user documents or broad Microsoft Office interoperability oracle were tested; synthetic parity only. x86_64 CLI/test build is validated; ARM is the native packaged pilot.

Native tar install uses `packaging/linux/install-pilot.sh`: no sudo, versioned user-local files, refuses unrelated collisions/unsafe archives, absolute desktop Exec/TryExec, leaves existing DeckCraft and MIME defaults alone. `--uninstall` removes only managed symlinks and preserves version folders for recovery. Inspect/remove specific retired folders manually.

## Installed pilot and final evidence

OmarchyMax user-local pilot `pilot-20261029-r1` is installed under `~/.local/share/omadeck/versions/`; `current` and `~/.local/bin/{omadeck,omadeck-cli}` point there. Desktop entry `~/.local/share/applications/io.github.ustaaa.OmaDeck.desktop` has absolute TryExec and `Exec=/usr/bin/env OMADECK_WAYLAND=1 ... %F`. Launched through `gio launch` with the synthetic saved presentation, actual PID **443241**, Hyprland **xwayland:false**. Both temporary validation ports **18800/18801 are closed**. Existing OmaGrid/Word processes were untouched. Screenshot and process/desktop evidence: `dist/validation/native/03-installed-native.png`, `installed.json` (ignored generated evidence, also on the Mac checkout).

Native sample reopening reports **Arial is unavailable on OmarchyMax** and uses upstream's similar-font fallback; no document font substitutions or host font installation were made. This is distinct from the synthetic CLI parity cases, whose open-warning lists were empty. The pilot is not a claim that proprietary fonts are installed.

ARM tar: `dist/release/omadeck-0.12.0-linux-aarch64.tar.gz`, SHA-256 `c3423bf36e6f32ba2b28bb24faa7f5dede299267b768813527a20a756464588c`. Contains original app licenses/NOTICE/attribution plus the font pack's OFL licenses (including Inter). Installer dry run and installation both passed. Native build log `~/.cache/omadeck-build/arm-pilot.log`; package log `package.log`.

Final PDF remainder: **16 passed**, one known baseline failure explicitly filtered from that separate run. Full wasm-safe target check: **23 crates passed**. Full-workspace aggregate CI still records the unchanged font-dependent PDF failure; all other workspace tests, strict native clippy, fmt, assets, layering and PDF boundary checks passed. Source changes are confined to UI/host identity, theme, tests, metadata and packaging; lower engine/document/render/font layers have zero diff from stable baseline.
