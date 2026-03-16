import { invoke } from "@tauri-apps/api/core";

export interface StartOptions {
  /**
   * If `true`, the session won't share cookies with Safari (no SSO, no
   * password manager autofill). Apple-only; ignored on Android.
   * Defaults to `false`.
   */
  ephemeral?: boolean;
}

/**
 * Start an in-app authentication session.
 *
 * Uses ASWebAuthenticationSession on Apple platforms and Chrome Custom Tabs
 * on Android.
 *
 * @param authUrl - Full authorization URL (e.g., OIDC authorize endpoint with PKCE params)
 * @param callbackUrlScheme - Just the scheme portion (e.g., `"myapp"`), not a full URL
 * @param options - Optional configuration
 * @returns The full callback URL including query params (code, state, etc.)
 * @throws `"user_cancelled"` if the user dismissed the auth session
 */
export async function start(
  authUrl: string,
  callbackUrlScheme: string,
  options?: StartOptions,
): Promise<string> {
  return await invoke<string>("plugin:auth-session|start", {
    authUrl,
    callbackUrlScheme,
    ephemeral: options?.ephemeral,
  });
}
