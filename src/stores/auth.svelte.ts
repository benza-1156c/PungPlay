import { commands, events } from "$lib/generated/bindings";

export const auth = $state({
  status: "idle" as "idle" | "logging_in" | "authenticated" | "error",
  err: null as string | null,
});

export async function login() {
  if (auth.status === "logging_in") return;

  auth.status = "logging_in";
  auth.err = null;

  const res = await commands.loginYoutube();
  if (res.status === "error") {
    auth.status = "error";
    auth.err = res.error;
  }
}

export async function logout() {
  await commands.logoutYoutube();
  auth.status = "idle";
  auth.err = null;
}

export async function initAuth() {
  const isAuthRes = await commands.isAuthenticated();
  if (isAuthRes.status === "ok" && isAuthRes.data) {
    auth.status = "authenticated";
  }

  return await events.appEvent.listen((e) => {
    if (e.payload.domain !== "auth") return;

    const authData = e.payload.data;
    if (authData.action === "success") {
      auth.status = "authenticated";
      auth.err = null;
    } else if (authData.action === "closed") {
      if (auth.status === "logging_in") {
        auth.status = "idle";
      }
    } else if (authData.action === "error") {
      auth.status = "error";
      auth.err = authData.payload;
    }
  });
}
