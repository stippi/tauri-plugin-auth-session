import { invoke } from "@tauri-apps/api/core";

export interface StartOptions {
  /**
   * If `true`, the session won't share cookies with Safari (no SSO, no
   * password manager autofill). Defaults to `false`.
   */
  ephemeral?: boolean;
}

/**
 * Start an ASWebAuthenticationSession on macOS or iOS.
 *
 * @param authUrl - Full authorization URL (e.g., an OIDC authorize endpoint with PKCE params)
 * @param callbackUrlScheme - Just the scheme portion (e.g., `"myapp"`), not a full URL
 * @param options - Optional configuration
 * @returns The full callback URL including query params (code, state, etc.)
 * @throws `"user_cancelled"` if the user dismissed the auth sheet
 */
export async function start(
  authUrl: string,
  callbackUrlScheme: string,
  options?: StartOptions,
): Promise<string> {
  return await invoke<string>("plugin:apple-auth|start", {
    authUrl,
    callbackUrlScheme,
    ephemeral: options?.ephemeral,
  });
}
