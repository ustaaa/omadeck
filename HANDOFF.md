# OmaDeck engineering handoff

## Development lifecycle

OmaCraft coordinates exact revisions, independent reviews and cross-platform evidence at
https://github.com/ustaaa/omacraft. This repository keeps its own upstream history.
Use origin for the Oma fork and upstream for the original project. Work on a topic branch,
review the diff, run relevant gates, and open a draft pull request. Do not automatically
publish releases or overwrite another device's dirty checkout.

## Current pilot (2026-10-30)

Linux ARM64 and x86_64 pilot binaries have been built. Native Wayland launch and sample
rendering were checked at display scales 1.6 and 1.5 respectively. This is not full
keyboard, IME, clipboard, accessibility or document-compatibility certification.
The ribbon and command layout are retained. Appearance must not modify document state.
macOS build workflows are experimental and manual; no Oma macOS distribution is certified.

Full Linux quality gates passed during pilot preparation. Fresh synthetic corpus comparisons
passed for Grid (3 cases), Word (2) and Deck (2); Notes CLI smoke passed 20 checks.
A previous x86 Notes UI test process terminated with SIGSEGV at four test threads.
All 121 UI tests then passed serially and under a debugger; full serial CI passed.
The cause remains unresolved. Do not describe reruns as a fix or hide this risk.

## Next review

Review downstream identity, theme isolation, font reload, package/updater separation,
and the declared support matrix. Revalidate at the exact reviewed commit and record
compiler, target, commands, result, skipped checks and logs. Keep private machine paths,
notebooks, credentials and infrastructure inventory out of tracked reports.

### OmaCraft brand polish (2026-10-30)
Suite links and documentation now use ustaaa/omacraft. Original application icon
regenerated in the shared dark-tile/paper style, with a distinct application accent.
PNG/SVG/ICO structure (where applicable), deterministic regeneration and whitespace
checks passed. No engine, document, command, dependency or version changes.
Installed-app validation of these new assets is still pending; previous runtime
evidence does not cover this polish. Existing CI limitations remain open.

### Suite review follow-up (2026-10-31)
Linux pilot policy documents native Wayland and `OMADECK_X11=1`; existing
`DECKCRAFT_WAYLAND` remains accepted. Recovery now uses the OmaDeck XDG data path;
legacy config recovery entries are copied without overwriting, with uid collisions
assigned a fresh recoverable id while the old files remain. Added a focused migration
test. Formatting and diff checks pass; Rust test execution could not access the Cargo
git checkout outside the writable workspace.

### Candidate 2 review fixes (2026-10-31)
Theme root priority/recovery and exact `OMADECK_X11=1` handling now have focused tests.
Legacy Deck recovery migration skips entries with a held owner lock in the old folder;
engine and app tests cover real lock files and migration. Cargo could not start tests
because the shared Cargo git DB is unwritable from this workspace; formatting passed.

### Upstream variable isolation by Claude (2026-10-31; branch claude/identity-env)
Preferences, recovery and the control port now read only `OMADECK_NO_PREFS`,
`OMADECK_NO_RECOVERY` and `OMADECK_CONTROL_PORT`. The `DECKCRAFT_` fallbacks are gone,
so values exported for an upstream DeckCraft install cannot switch off OmaDeck
preferences or recovery or bind the same control port. `DECKCRAFT_WAYLAND` stays as the
documented backend alias. A unit test proves upstream names are ignored.
