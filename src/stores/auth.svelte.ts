import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type AuthStatus = "idle" | "logging_in" | "authenticated" | "error";

export interface AuthState {
  authStatus: AuthStatus;
  err: string | null;
}

export const AUTH_EVENTS = {
  LOGIN_SUCCESS: "login-success",
  LOGIN_CLOSED: "login-closed",
} as const;

class AuthStore implements AuthState {
  authStatus = $state<AuthStatus>("idle");
  err = $state<string | null>(null);

  private unlistenSuccess: UnlistenFn | null = null;
  private unlistenClosed: UnlistenFn | null = null;
  private isInitialized = false;

  // Helper getters for convenience
  get isLoggedIn() {
    return this.authStatus === "authenticated";
  }

  get isLoggingIn() {
    return this.authStatus === "logging_in";
  }

  get error() {
    return this.err;
  }

  /**
   * Initialize event listeners for login status from Tauri backend.
   * Call this on mount or in the root layout.
   */
  async init(onSuccess?: () => void) {
    if (this.isInitialized) return;
    this.isInitialized = true;

    try {
      this.unlistenSuccess = await listen(AUTH_EVENTS.LOGIN_SUCCESS, () => {
        this.authStatus = "authenticated";
        this.err = null;
        console.log("[Auth] YouTube Music login successful!");
        onSuccess?.();
      });

      this.unlistenClosed = await listen(AUTH_EVENTS.LOGIN_CLOSED, () => {
        if (this.authStatus === "logging_in") {
          this.authStatus = "idle";
        }
        console.log("[Auth] YouTube Music login window closed.");
      });
    } catch (e: any) {
      this.authStatus = "error";
      this.err = typeof e === "string" ? e : e?.message || "Failed to register auth event listeners";
      console.error("[Auth] Failed to register event listeners:", e);
    }
  }

  /**
   * Open YouTube Music Google Login window via Tauri WebView.
   */
  async login() {
    if (this.authStatus === "logging_in") return;

    this.authStatus = "logging_in";
    this.err = null;

    try {
      await invoke("login_youtube");
    } catch (error: any) {
      this.authStatus = "error";
      this.err =
        typeof error === "string"
          ? error
          : error?.message || "Failed to launch login window";
      console.error("[Auth] Failed to invoke login_youtube:", error);
    }
  }

  /**
   * Log out and reset state back to idle.
   */
  logout() {
    this.authStatus = "idle";
    this.err = null;
  }

  /**
   * Clean up event listeners.
   */
  destroy() {
    if (this.unlistenSuccess) {
      this.unlistenSuccess();
      this.unlistenSuccess = null;
    }
    if (this.unlistenClosed) {
      this.unlistenClosed();
      this.unlistenClosed = null;
    }
    this.isInitialized = false;
  }
}

export const auth = new AuthStore();
