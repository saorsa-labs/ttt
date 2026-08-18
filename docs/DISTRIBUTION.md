# Shipping ttt

## What is automated today

Tag `vX.Y.Z` on `main`. GitHub Actions builds Linux, Windows, and both macOS arches, then attaches binaries to the GitHub Release. Anyone can download and run those.

That is the real "play on any desktop" path.

## What cannot be fully automated

App stores always need a human once:

1. **Apple Developer Program** (~$99/year) and an App Store Connect API key (or App Store Connect login).
2. **Google Play Console** ($25 one-time) and a service account JSON.
3. **Signing keys** stored as GitHub secrets. Lost keys mean you cannot update the same app.
4. **First listing**: name, screenshots, age rating, privacy text. Review is a person at Apple/Google.
5. **Notarization / Play signing** after that can run in CI (Fastlane, `xcrun notarytool`, Play Developer API).

Nobody can skip those five for a store listing. After they exist, CI can ship every later build.

## Platform reality (August 2026)

| Target | GPUI | How people get it |
| --- | --- | --- |
| Linux | Official | GitHub Release binary |
| Windows | Official | GitHub Release `.exe` |
| macOS | Official | GitHub Release binary. Optional later: notarized `.app` / Homebrew |
| Web / WASM | Not on the GPUI roadmap | Would need a different UI |
| iOS | Not official. Community `gpui-mobile` exists | Needs Apple account + Xcode/macOS CI runner + TestFlight |
| Android | Not official. Community `gpui-mobile` exists | Needs Play account + NDK CI |

GPUI is Zed's desktop GPU UI. It is the right stack for Windows / Mac / Linux. It is the wrong stack if the first goal is "install from the App Store tomorrow."

## Recommended sequence

1. **Now:** GitHub Releases for the four desktop binaries. One tag, no store accounts.
2. **Optional desktop polish:** `cargo-dist` installers (shell, PowerShell, Homebrew) and Apple notarization once a Developer account exists.
3. **Phones later, only if we still want native GPUI:** evaluate `gpui-mobile` against a rewrite in something that already ships mobile (Dioxus, egui + eframe, Tauri 2). Do not start store paperwork until that spike builds on a device.
4. **Stores:** Fastlane in GitHub Actions, secrets only, no one clicking consoles after the first listing.

## Secrets we will need later (do not invent them now)

- `APPLE_ID` / `APP_STORE_CONNECT_API_KEY` / `APPLE_CERTIFICATE_P12`
- `GOOGLE_PLAY_SERVICE_ACCOUNT_JSON`
- Android keystore + passwords

Put those in GitHub Environments, not in the repo.
