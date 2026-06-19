# hplan

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

The timeline draws each housing as a row and each stay as a colored bar. A
housing's name turns red with a ⚠ when its capacity is exceeded on any visible
day. The red vertical line marks today.

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
| `src/app.rs` | eframe `App`: side-panel editors + central timeline |
| `src/main.rs` | Desktop entry point |
| `src/lib.rs` | Shared library + Android `android_main` entry |
