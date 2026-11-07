import { commands, events, type ProfileSummary } from "$lib/generated/bindings";

export const auth = $state({
  status: "idle" as "idle" | "logging_in" | "authenticated" | "error",
  err: null as string | null,
  profiles: [] as ProfileSummary[],
  activeProfile: null as ProfileSummary | null,
});

export async function loadProfiles() {
  const res = await commands.listProfiles();
  if (res.status === "ok") {
    auth.profiles = res.data;
    auth.activeProfile =
      res.data.find((p) => p.is_active) ?? res.data[0] ?? null;
    if (auth.activeProfile) {
      auth.status = "authenticated";
    } else {
      auth.status = "idle";
    }
  }
}

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
  auth.status = "idle";
  auth.err = null;
  auth.profiles = [];
  auth.activeProfile = null;
}

export async function switchProfile(profileId: string) {
  const res = await commands.switchProfile(profileId);
  if (res.status === "ok") {
    await loadProfiles();
  }
  return res;
}

export async function initAuth() {
  await loadProfiles();

  return await events.appEvent.listen((e) => {
    if (e.payload.domain !== "auth") return;

    const authData = e.payload.data;
    if (authData.action === "success") {
      auth.status = "authenticated";
      auth.err = null;
      loadProfiles();
    } else if (authData.action === "closed") {
      if (auth.status === "logging_in") {
        auth.status = auth.activeProfile ? "authenticated" : "idle";
      }
    } else if (authData.action === "error") {
      auth.status = "error";
      auth.err = authData.payload;
    }
  });
}
