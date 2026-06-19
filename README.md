# housingplanner

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

You can also **save/load an explicit `.json` file** with the buttons in the top
bar (💾 Save… / 📂 Load…), e.g. to share a plan or keep backups. The native file
dialog uses Win32 on Windows and the xdg-desktop-portal on Linux (no GTK build
dependency). Loading replaces the current plan.

## Run (desktop: Linux / Windows / macOS)

```sh
cargo run --release
```

### Top bar controls
- **From** — first date shown.
- **Days** — how many days are visible.
- **Zoom** — width of each day column.
- **Today** / **Fit to stays** — jump the view.

### Timeline gestures
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
| `src/app.rs` | eframe `App`: side-panel editors + central timeline + About window |
| `src/licenses.rs` | Embedded app + dependency license attribution |
| `src/main.rs` | Desktop entry point (incl. `--licenses` flag) |
| `src/lib.rs` | Shared library + Android `android_main` entry |
