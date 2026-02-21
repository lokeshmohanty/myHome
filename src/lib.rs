mod app;
pub mod db;
pub mod modules;

// ── Android entry point ───────────────────────────────────────────────────────
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    // Set the Android app handle so Slint's backend can use it
    slint::android::init(app).unwrap();

    // Spawn tokio runtime manually (no #[tokio::main] on mobile)
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        self::app::run().unwrap();
    });
}

// ── iOS entry point ───────────────────────────────────────────────────────────
// iOS uses the standard C main(), Slint handles UIKit setup internally
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        self::app::run().unwrap();
    });
    0
}
