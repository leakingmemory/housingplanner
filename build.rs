//! Build script. On Windows it embeds the application icon into the `.exe` so it
//! shows up in Explorer / the taskbar. On every other host this is a no-op (the
//! `winresource` dependency is only pulled in for Windows targets).

fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        // Don't fail the build if the resource compiler is unavailable.
        let _ = res.compile();
    }
}
