//! Tauri v2 plugin wrapping Apple's ASWebAuthenticationSession for macOS and iOS.
//!
//! Presents an in-app authentication sheet (managed by ASWebAuthenticationSession)
//! instead of opening the system browser, satisfying Apple App Store Guideline 4 (Design).
//!
//! # Usage
//!
//! ```rust,no_run
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_apple_auth::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;

/// Initialize the plugin.
///
/// On Apple platforms (macOS / iOS), this registers the `start` command which
/// opens an ASWebAuthenticationSession. On other platforms the command returns
/// an error indicating the API is unavailable.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("apple-auth")
        .invoke_handler(tauri::generate_handler![start])
        .build()
}

// ---------------------------------------------------------------------------
// Apple implementation
// ---------------------------------------------------------------------------

/// Start an ASWebAuthenticationSession and return the callback URL.
///
/// # Arguments
/// * `auth_url` — Full authorization URL (e.g., an OIDC authorize endpoint with PKCE params)
/// * `callback_url_scheme` — Just the scheme portion (e.g., `"myapp"`), not a full URL
/// * `ephemeral` — If `true`, the session won't share cookies with Safari (no SSO,
///   no password manager autofill). Defaults to `false` if omitted.
///
/// # Returns
/// * `Ok(callback_url)` — The full callback URL including query params (code, state, etc.)
/// * `Err("user_cancelled")` — User dismissed the auth sheet
/// * `Err(message)` — Other error
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[tauri::command]
async fn start(
    auth_url: String,
    callback_url_scheme: String,
    ephemeral: Option<bool>,
) -> Result<String, String> {
    apple::start_session(auth_url, callback_url_scheme, ephemeral.unwrap_or(false)).await
}

// ---------------------------------------------------------------------------
// Non-Apple stub
// ---------------------------------------------------------------------------

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[tauri::command]
async fn start(
    _auth_url: String,
    _callback_url_scheme: String,
    _ephemeral: Option<bool>,
) -> Result<String, String> {
    Err("ASWebAuthenticationSession is only available on Apple platforms (macOS / iOS)".to_string())
}
