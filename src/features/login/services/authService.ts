import { invoke } from "@tauri-apps/api/core";

export function isAuthenticated(): Promise<boolean> {
  return invoke<boolean>("is_authenticated");
}

export function beginLogin(): Promise<string> {
  return invoke<string>("begin_login");
}

export function confirmLogin(): Promise<void> {
  return invoke<void>("confirm_login");
}
