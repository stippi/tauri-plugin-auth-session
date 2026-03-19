// index.ts
import { invoke } from "@tauri-apps/api/core";
async function start(authUrl, callbackUrlScheme, options) {
  return await invoke("plugin:auth-session|start", {
    authUrl,
    callbackUrlScheme,
    ephemeral: options?.ephemeral
  });
}
export {
  start
};
