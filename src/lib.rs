//! housingplanner — a cross-platform housing & stay planner.
//!
//! The desktop binary lives in `main.rs`; on Android the entry point is the
//! `android_main` export below. Both share the same [`app::PlannerApp`].

pub mod app;
pub mod i18n;
pub mod licenses;
pub mod model;
pub mod timeline;

/// Android entry point. Invoked by the `android-activity` glue (via the
/// `NativeActivity` declared in the app manifest) when building with
/// `cargo-apk` / `xbuild`.
#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        event_loop_builder: Some(Box::new(move |builder| {
            builder.with_android_app(app);
        })),
        ..Default::default()
    };

    eframe::run_native(
        "Housing Planner",
        options,
        Box::new(|cc| Ok(Box::new(crate::app::PlannerApp::new(cc)))),
    )
    .expect("failed to start Housing Planner");
}
