# housingplanner

<img src="assets/logo.png" alt="Housing Planner" width="420">

A small cross-platform desktop (and, with extra setup, Android) app for
planning **who stays where, and when**. You register housings, people and
groups, assign stays with arrival/departure dates, and see it all laid out on a
timeline so overlaps and over-capacity jump out visually.

Built in Rust with [`egui`](https://github.com/emilk/egui) / `eframe`.

## Concepts

- **Housing** — a place to stay, with a `capacity`.
- **Group** — a named, colored set of people (a family, a team, …).
- **Person** — optionally belongs to a group; inherits the group's color.
- **Stay** — assigns a person *or* a whole group to a housing for an
  `arrival → departure` date range. Arrival is the first night, departure is the
  checkout day.

The timeline draws each housing as a row and each stay as a colored bar.
Overlapping stays in the same housing are stacked into sub-lanes so each
occupant's color stays visible (no bar hides another). Where occupancy exceeds
the housing's capacity, that date span is overlaid with a diagonal red **hatch**
— a clear double-booking indicator. A housing's name also turns red with a ⚠
when over capacity, and the red vertical line marks today.

If the same person or group is booked in **two locations at the same time**
(impossible to satisfy), each involved bar gets an amber border and a "!" badge.
Groups are expanded to their members, so a person booked individually while their
group is booked elsewhere is flagged too.

Data is saved automatically via `eframe`'s built-in storage (per-OS app data
directory), so it persists across runs on every platform without managing file
paths.

You can also **save/load an explicit `.json` file** with the top-bar buttons:
**💾 Save** writes to the file you currently have open (no dialog); **Save As…**
prompts for a location; **📂 Load…** opens one. The native file dialog uses Win32
on Windows and the xdg-desktop-portal on Linux (no GTK build dependency). The
current file name and an unsaved-changes dot (●) are shown in the top bar.

The app **reopens the last file on startup**, and **warns before closing if there
are unsaved changes** (Save / Discard / Cancel). Loading replaces the current plan.

## Run (desktop: Linux / Windows / macOS)

```sh
cargo run --release
```

## Install (Snap, Linux)

Released to the Snap Store's edge channel:

```sh
sudo snap install housingplanner --edge
```

Build the snap locally (needs `snapcraft` + LXD):

```sh
snapcraft --use-lxd
sudo snap install ./housingplanner_*.snap --dangerous
```

The snap is a strictly-confined desktop app (uses the `gnome` extension for
graphics/fonts and the `home` interface for the Save/Load dialogs). Publishing to
the store is done by the `snapcraft.yml` workflow when a `v*` tag is pushed; that
requires the `housingplanner` name registered in the store and a `SNAPCRAFT_LOGIN`
repository secret. The snap version in `snap/snapcraft.yaml` is kept in sync with
`Cargo.toml` (enforced by `check-versions.yml`).

### Tabs
- **📊 Overview** — the full timeline of all housings and stays.
- **👥 Groups** — pick a group; edit its name/color, add/remove members (attach
  existing people or create new ones), manage its stays, and see a timeline of
  just that group across the housings it uses.
- **🧍 Persons** — pick a person; edit name/group, manage their stays, and see a
  timeline of their whereabouts (own stays **plus** any group they belong to).
- **🏠 Housings** — pick a housing; edit capacity/notes, manage its stays, and see
  that single housing's timeline (with the over-capacity hatch).
- **📜 Changelog** — an auditable journal of every edit (create/rename/delete,
  capacity, membership, stays, …), with a localized description and timestamp.
  The journal is **saved inside the plan file** and survives reload. **↩ Undo
  last change** reverts the most recent change (for changes made in the current
  session) and logs an `Undo` entry referencing the change's id. Loading a plan
  saved before this feature adds a "no change history" entry. Old apps still read
  newer files (the journal is an ignored extra field), and this app still reads
  pre-journal files.

### Top bar controls
- **From** — first date shown.
- **Days** — how many days are visible.
- **Zoom** — width of each day column.
- **Today** / **Fit to stays** — jump the view.
- **🌐 Language** — switch between English, Svenska (Swedish), Norsk bokmål,
  Norsk nynorsk (Norwegian), Davvisámegiella (Northern Sami), Dansk (Danish),
  Українська (Ukrainian), Deutsch (German), Français (French), Italiano (Italian),
  Español (Spanish), Nederlands (Dutch), Русский (Russian), Íslenska (Icelandic),
  Føroyskt (Faroese — best-effort, needs review) and Kalaallisut (Greenlandic — a
  best-effort stub, mostly English; needs a native speaker). Remembered between
  runs; defaults to your system locale (`LANG` / `LC_*`).

The top-bar controls and the timeline gestures below apply to every tab's
timeline.

### Timeline gestures
- **Hover** a stay bar for a tooltip with who, the housing, and the from/to dates
  (plus nights, group headcount, and a double-booking warning where relevant).
- **Drag** the timeline left/right to pan through time.
- **Ctrl/Cmd + scroll** (or pinch on a trackpad) over the timeline to zoom,
  anchored on the date under the pointer. Plain scroll moves the housing list
  vertically.

## Licenses / attribution

MIT, BSD and most other dependency licenses require their full text and copyright
notices to be distributed with the binary. Those notices are collected from the
crate sources by [`cratelist`](https://github.com/leakingmemory/cratelist) into
`DEPENDENCIES_LICENSE`, committed gzipped (`DEPENDENCIES_LICENSE.gz`), and
**embedded into the binary** (decompressed on demand with `flate2`). View them via:

- **In the app:** the **ℹ About** button (top-right) opens an About / Licenses
  window with this app's license and the full third-party list. This is the only
  attribution surface on Android.
- **Command line:** `housingplanner --licenses` prints everything to stdout.
  (On Windows release builds there is no attached console, so use the About
  window there.)

The `.github/workflows/update_licenses.yml` workflow regenerates the list on every
`Cargo.lock` change. To regenerate manually:

```sh
cargo fetch
CRATES_DIR=$(ls -d ~/.cargo/registry/src/index.crates.io-* | head -n1)
cratelist Cargo.lock --license-contents "$CRATES_DIR" > DEPENDENCIES_LICENSE
gzip -fk DEPENDENCIES_LICENSE
```

## Branding / assets

Logo and icon sources live in `assets/`:

- `assets/icon.svg` / `assets/logo.svg` — the editable vector sources (a house
  whose interior is the app's colored timeline bars).
- `assets/icon-*.png`, `assets/logo.png`, `assets/icon.ico` — generated raster
  exports. The window/taskbar icon is embedded from `assets/icon-256.png`; on
  Windows the `.exe` icon is embedded from `assets/icon.ico` via `build.rs`.

Regenerate the raster exports after editing the SVGs:

```sh
cd assets
for s in 16 32 48 64 128 256 512; do rsvg-convert -w $s -h $s icon.svg -o icon-$s.png; done
rsvg-convert -w 900 -h 240 logo.svg -o logo.png
python3 -c "from PIL import Image; Image.open('icon-256.png').save('icon.ico', \
  sizes=[(16,16),(32,32),(48,48),(64,64),(128,128),(256,256)])"
```

The same PNGs are the source for Android launcher densities (wiring up
`cargo-apk` mipmaps is left for when the Android build is set up).

### Desktop launcher (manual install on Linux)

`assets/housingplanner.desktop` is a freedesktop entry for a non-snap install
(the snap ships its own under `snap/gui/`). After putting the `housingplanner`
binary on your `PATH`:

```sh
install -Dm644 assets/housingplanner.desktop ~/.local/share/applications/housingplanner.desktop
install -Dm644 assets/icon-256.png ~/.local/share/icons/hicolor/256x256/apps/housingplanner.png
update-desktop-database ~/.local/share/applications 2>/dev/null || true
```

The window sets its app id to `housingplanner` (matching the file's
`StartupWMClass`), so the launcher icon associates with the running window.

## Development

Code is formatted with `cargo fmt` and CI (`.github/workflows/master.yml`) runs
`cargo fmt -- --check` on every push/PR to `master`. Enable the bundled
pre-commit hook so unformatted commits are blocked locally:

```sh
git config core.hooksPath .githooks
```

The pre-commit hook (and the `check-translations.yml` CI workflow) also run
`scripts/check_translations.py`, which fails if any UI string used via
`tr(lang, "…")` is missing from one of the language tables in `src/i18n.rs` — so
new text can't silently fall back to English in another language. Run it directly
with `python3 scripts/check_translations.py`.

## Android (optional, extra toolchain required)

The code is already Android-ready: app logic is in the library crate and
`android_main` in `src/lib.rs` is the entry point. To actually build an APK you
need the Android NDK and a build tool. With
[`cargo-apk`](https://github.com/rust-mobile/cargo-apk):

```sh
rustup target add aarch64-linux-android      # (requires rustup)
cargo install cargo-apk
export ANDROID_NDK_HOME=/path/to/android-ndk
cargo apk run --lib
```

> Note: this machine uses the Gentoo Rust toolchain (no `rustup`). Cross-
> compiling to Android needs `rustup` (or a manually installed
> `aarch64-linux-android` std) plus the NDK; set that up before the commands
> above.

## Project layout

| File | Purpose |
|------|---------|
| `src/model.rs` | Data types (`Housing`, `Group`, `Person`, `Stay`, `Plan`) + helpers |
| `src/timeline.rs` | The Gantt-style timeline rendering |
| `src/app.rs` | eframe `App`: Overview/Groups/Persons/Housings/Changelog tabs + per-tab timeline + About window |
| `src/journal.rs` | Change-journal engine: diff → entries, session-undo inverses, localized descriptions |
| `src/licenses.rs` | Embedded app + dependency license attribution |
| `src/main.rs` | Desktop entry point (incl. `--licenses` flag) |
| `src/lib.rs` | Shared library + Android `android_main` entry |
