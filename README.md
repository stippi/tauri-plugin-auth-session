# tauri-plugin-auth-session

Tauri v2 plugin for in-app OAuth authentication on Apple and Android platforms.

- **macOS / iOS:** [ASWebAuthenticationSession](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsession)
- **Android:** [Chrome Custom Tabs](https://developer.chrome.com/docs/android/custom-tabs)

## Platform Support

| Platform | Mechanism | SSO |
|----------|-----------|-----|
| macOS 10.15+ | ASWebAuthenticationSession | Safari cookies |
| iOS 13+ | ASWebAuthenticationSession | Safari cookies |
| Android 7+ (API 24) | Chrome Custom Tabs | Chrome cookies |
| Windows / Linux | Stub (returns error) | N/A |

On unsupported platforms the plugin registers without error but the `start` command returns an `Err`.

## Installation

### Rust

Add to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-auth-session = { git = "https://github.com/yanqianglu/tauri-plugin-auth-session" }
```

Register in `src-tauri/src/lib.rs`:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_auth_session::init())
    // ...
```

### Permissions

Add to your capability file (e.g., `src-tauri/capabilities/default.json`):

```json
{
  "permissions": ["auth-session:default"]
}
```

### Android Setup

The plugin declares `AuthSessionActivity` in its own manifest. You must add an intent filter with your app's callback scheme in your `AndroidManifest.xml`:

```xml
<activity
    android:name="app.tauri.auth_session.AuthSessionActivity"
    android:exported="true"
    tools:node="merge">
    <intent-filter>
        <action android:name="android.intent.action.VIEW" />
        <category android:name="android.intent.category.DEFAULT" />
        <category android:name="android.intent.category.BROWSABLE" />
        <data android:scheme="myapp" />
    </intent-filter>
</activity>
```

Replace `myapp` with your app's custom URL scheme.

## Usage

```typescript
import { invoke } from "@tauri-apps/api/core";

const callbackUrl = await invoke<string>("plugin:auth-session|start", {
  authUrl: "https://your-idp.com/authorize?client_id=...&redirect_uri=...&code_challenge=...",
  callbackUrlScheme: "myapp",
});
// callbackUrl = "myapp://callback?code=...&state=..."
```

Or with the guest-js package:

```typescript
import { start } from "tauri-plugin-auth-session-api";

const callbackUrl = await start(authorizeUrl, "myapp");

// Apple-only: use ephemeral mode (no SSO, no shared cookies):
const callbackUrl = await start(authorizeUrl, "myapp", { ephemeral: true });
```

## How It Works

1. Your app builds an OAuth/OIDC authorize URL with PKCE parameters
2. Call `start` with the URL and your app's custom URL scheme
3. The plugin opens an in-app auth session (ASWebAuthenticationSession on Apple, Chrome Custom Tab on Android)
4. User authenticates
5. The identity provider redirects to your custom scheme
6. The plugin captures the redirect and returns the full callback URL
7. Your app extracts the authorization code and exchanges it for tokens

## Error Handling

| Error | Meaning |
|-------|---------|
| `"user_cancelled"` | User dismissed the auth session |
| `"Invalid auth URL: ..."` | The provided URL couldn't be parsed |
| `"Auth session error: ..."` | Platform-specific error |
| `"No browser available"` | No Custom Tabs-capable browser on Android |
| `"Not available on this platform"` | Called on Windows/Linux |

## Notes

- **`ephemeral`** option controls `prefersEphemeralWebBrowserSession` on Apple. When `false` (default), the session shares cookies with Safari for SSO. Ignored on Android (Custom Tabs always share Chrome cookies).
- On **macOS**, ASWebAuthenticationSession opens a system-managed authentication window.
- On **iOS**, it presents a modal sheet anchored to the app's key window.
- On **Android**, it opens a Chrome Custom Tab within the app. If Chrome is unavailable, falls back to the default browser.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
