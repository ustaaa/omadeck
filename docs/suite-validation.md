# OmaDeck validation against upstream

Baseline: `deckcraft` v0.12.0, `29f0f7772c47aa2bfb74fcbb0c4d3b1503300849`. The manifest pins this source identity; binary provenance must be supplied by the build log (a version string alone cannot attest a commit). No engine, document model, import/export or render changes are part of this adaptation.

## Executable corpus

Run upstream and fork on the **same host**, with the same `CRAFT_FONTS_DIR` at build time and the same installed fonts at runtime. Do not rebuild or replace either binary during a run. Python 3.9+ standard library only:

```sh
python3 -m unittest discover -s tools/omarchy-validation -p 'test_*.py'
python3 tools/omarchy-validation/corpus.py \
  --upstream /absolute/path/deckcraft-cli \
  --fork /absolute/path/omadeck-cli \
  --output dist/validation/corpus-new-run
```

The output directory must not exist. Every command, stdout, stderr, source fixture, edited output, observation and rendered PNG is retained there. All fixtures are contributor-original synthetic text/geometry generated through upstream's command API; no office application or real user document is involved. Generated binaries/documents stay under ignored `dist/`, never in Git.

Each of two cases (native format and OOXML) creates a formatted document/presentation with a table, saves it, opens the **same input** in upstream and fork, edits, verifies undo removes the edit and redo restores it, saves another file, and reopens in a fresh CLI process. Input is made read-only and its hash is checked after each implementation; binary hashes are recorded and checked again at the end. The runner compares full command results, inspected content/style data and exact PNG hashes for every page/slide. Word renders use en-US region and UTC. Only per-save Word created/modified timestamps and evidence-directory prefixes in top-level file paths are normalized. Raw results remain available.

Failures are nonzero exit, malformed/missing responses, warning text on stderr, nonempty explicit warning/loss fields, expected content missing, changed undo/redo results, lost tables, different styles/semantics, different rendered pixels, input mutation or binary replacement. Deck open responses must expose `lostSlides`, `damagedMedia`, and `missingFonts`; loss/damage must be empty. Missing fonts are retained and compared, not suppressed or claimed installed. The Word file API does **not** expose a general data-loss/no-oracle field: absence is not an interoperability oracle. Any future explicit `no_oracle`/warning field is checked, never discarded.

## Scope and acceptance gates

- Corpus pass proves no observed fork regression in these two small generated cases on the named host. It does not prove Microsoft Office equivalence, complex external document fidelity, broad OOXML coverage or native UI correctness.
- Preserve UI/document font role tests: theme changes must leave document font chains, content colours and render engine untouched; IME/text events must survive a pending font refresh.
- Run the repo's required Rust CI (including formatting, clippy, tests, layers, asset checks, WASM where supported); record any skipped target explicitly.
- On actual Omarchy: inspect light/dark palette transitions, manual appearance mode, invalid/missing theme fallback, font changes while typing, notes/comments, save/reopen, file picker and clipboard. CLI PNG parity cannot validate egui notes/comments or physical keyboard behavior.
- Check desktop ID, icon, MIME association, isolated config/recovery namespaces, CLI help/version, window title, and disabled upstream updater. Native tar is the first runtime acceptance target; Flatpak theme-file access remains a separate sandbox gate.
- Rehearse upstream integration against a genuinely divergent upstream revision in an isolated checkout before merging, then rerun these gates. The manifest baseline changes only with an intentional upgrade.

## Recorded results

Harness fault-injection tests and real corpus outcomes are recorded below as execution occurs. A checked-in script is not a completed runtime gate.

- 2026-10-29, UrazHalo amd64: **2/2 corpus cases passed** (`dist/validation/corpus-amd64-r2/report.json`). Upstream official v0.12.0 binary versus locally built fork debug CLI; both use Deck craft-fonts pin `37827adc41fd165f9a9234ac40f94dbad353aa69`. Native `.deckcraft` and `.pptx`: detailed shape/content/notes/table observations, edit/undo/redo/save responses, and both slide PNG hashes matched. `lostSlides`, `damagedMedia`, and `missingFonts` were empty. Shared input and binary hashes remained unchanged. The earlier preflight/r1 failures were validator mistakes (active-slide scope for shape inspection and valid JSON null response), fixed without engine changes.
- Validator fault injection: 6 tests passed locally. UI input tests are separate from this CLI result; their status belongs in the build/test handoff.

The complete Deck CLI command catalog also matched byte-for-byte (`catalog.json`). The inherited Deck control service has no token authentication and is opt-in loopback; the pilot must start without a control port. Protocol authentication is unchanged by this appearance fork.

- New `appearance_input_tests` both passed in the UrazHalo UI test run: a pending desktop font change preserves physical text exactly once, preserves Japanese preedit/commit as one undo step, and applies after editor exit. The first full UI run had unrelated test setup/assertion failures still being repaired; do not infer full CI success from these two results. See final HANDOFF for the final suite status.

- Final UI run: **425 passed, 0 failed, 4 ignored**, including the two input/IME regressions. Full workspace excluding PDF: **2108 passed, 0 failed, 6 ignored**. PDF remainder: 16 passed; one font-dependent PDF test fails identically in pristine upstream and remains documented, not patched. See `HANDOFF.md` for detailed environment and log locations.
- Actual OmarchyMax native Wayland pilot installed and running; live palette/manual/last-good/save/reopen smoke passed using an isolated theme directory. Final normal launcher has no control port. Arial is absent on the native sample host; its visible upstream fallback warning is retained.
