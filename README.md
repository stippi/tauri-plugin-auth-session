# tauri-plugin-apple-auth

Tauri v2 plugin wrapping Apple's [ASWebAuthenticationSession](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsession) for macOS and iOS.

Presents an in-app authentication sheet instead of opening the system browser, satisfying **Apple App Store Guideline 4 (Design)**.

## Platform Support

| Platform | Status |
|----------|--------|
| macOS 10.15+ | Supported |
| iOS 13+ | Supported |
| Windows / Linux | Stub (returns error) |
| Android | Stub (returns error) |

On non-Apple platforms the plugin registers without error but the `start` command returns an `Err`, so you don't need `#[cfg]` guards in your app setup.

## Features

- In-app auth sheet via ASWebAuthenticationSession (no system browser redirect)
- Shares cookies with Safari for SSO and password manager support
- Works with any OAuth 2.0 / OIDC provider (Keycloak, Auth0, etc.)
- PKCE support (S256) — handle PKCE on the frontend, pass the final authorize URL
- Proper cancellation handling (`"user_cancelled"` error)

## Installation

### Rust

Add to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-apple-auth = { git = "https://github.com/yanqianglu/tauri-plugin-apple-auth" }
```

Register the plugin in your `src-tauri/src/lib.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_apple_auth::init())
        // ... other plugins
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Permissions

Add the permission to your capability file (e.g., `src-tauri/capabilities/default.json`):

```json
{
  "permissions": [
    "apple-auth:default"
  ]
}
```

### Frontend (TypeScript)

You can either copy the invoke call directly or use the guest-js bindings:

```typescript
import { invoke } from "@tauri-apps/api/core";

const callbackUrl = await invoke<string>("plugin:apple-auth|start", {
  authUrl: "https://your-idp.com/authorize?client_id=...&redirect_uri=...&code_challenge=...",
  callbackUrlScheme: "myapp",
});
// callbackUrl = "myapp://callback?code=...&state=..."
```

Or with the guest-js package:

```typescript
import { start } from "tauri-plugin-apple-auth-api";

const callbackUrl = await start(authorizeUrl, "myapp");

// Use ephemeral mode (no SSO, no shared Safari cookies):
const callbackUrl = await start(authorizeUrl, "myapp", { ephemeral: true });
```

## How It Works

1. Your app builds an OAuth/OIDC authorize URL with PKCE parameters
2. Call `start` with the URL and your app's custom URL scheme
3. The plugin opens an ASWebAuthenticationSession (in-app auth sheet)
4. User authenticates in the sheet
5. The identity provider redirects to your custom scheme
6. The plugin captures the redirect and returns the full callback URL
7. Your app extracts the authorization code and exchanges it for tokens

## Error Handling

The `start` command can return these errors:

| Error | Meaning |
|-------|---------|
| `"user_cancelled"` | User dismissed the auth sheet |
| `"Invalid auth URL: ..."` | The provided URL couldn't be parsed |
| `"Auth session error: ..."` | ASWebAuthenticationSession reported an error |
| `"ASWebAuthenticationSession is only available on Apple platforms (macOS / iOS)"` | Called on a non-Apple platform |

## Notes

- **`ephemeral`** option controls `prefersEphemeralWebBrowserSession`. When `false` (default), the session shares cookies with Safari for SSO and password manager autofill. Set to `true` for isolated sessions (e.g., multi-account support).
- On **macOS**, ASWebAuthenticationSession opens a separate authentication window managed by the system — this is the expected behavior and passes App Store review.
- On **iOS**, it presents a modal sheet anchored to the app's key window.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
