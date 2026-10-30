# Rotunda for the desktop

A small app that opens [Rotunda](https://myrotunda.com) in its own window, with its own
icon, on Mac, Windows and Linux. It holds no Rotunda code and no data: everything,
including sign-in, lives on myrotunda.com, so changes to the site show up here without
a new download.

**Download:** [myrotunda.com/download](https://myrotunda.com/download), or the latest
[release](https://github.com/LawClerkNobleLaw/rotunda-desktop/releases/latest).

## How it's built

[Tauri 2](https://v2.tauri.app). `src-tauri/src/main.rs` is the whole app.
GitHub Actions builds all three installers (`.github/workflows/build.yml`):

- **Try a build:** Actions → Build → Run workflow. Installers are attached to the run.
- **Release one:** bump `version` in `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml`,
  then push a tag: `git tag v1.0.1 && git push origin v1.0.1`.

Icons are generated at build time from `app-icon.svg`.

## Mac signing

Mac builds are signed and notarized once these repository secrets exist:
`APPLE_CERTIFICATE` (base64 of the Developer ID Application `.p12`),
`APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, `APPLE_ID`,
`APPLE_PASSWORD` (an app-specific password) and `APPLE_TEAM_ID`.
Until then the Mac app is unsigned, and Macs ask people to approve it once in
System Settings → Privacy & Security → Open Anyway.
