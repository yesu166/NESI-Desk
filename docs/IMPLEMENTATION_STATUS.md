# Implementation status

**Branch:** `feat/nesi-desk-foundation`  
**Release status:** `IN_PROGRESS`

Status labels describe the evidence available today. A code path is not considered physically verified just because it compiles.

| Area | Status | Evidence / limitation |
|---|---|---|
| React + TypeScript + Vite shell | `TESTED` | Windows CI typecheck, frontend tests, and Vite build have passed on earlier branch revisions; latest changes are awaiting CI |
| Tauri 2 + Rust command boundary | `IN_PROGRESS` | Native commands and minimal capability file exist; latest Rust CI must pass after the newest session changes |
| ADB tool detection | `IMPLEMENTED` | Detects executable and reports version/error |
| ADB device discovery/state | `IMPLEMENTED` | Parses actual `adb devices -l` output; authorization state is not fabricated |
| Installed app discovery | `IMPLEMENTED` | Lists third-party package IDs; display names/icons are currently approximated from package names |
| USB onboarding | `IMPLEMENTED` | Instructions, authorization guidance, refresh, and actionable missing-tool messaging |
| Wireless ADB pairing/connect | `IMPLEMENTED` | Explicit pairing-address/code and connection-address flow; requires Android Wireless debugging to be enabled by the user |
| Single-app virtual display request | `IMPLEMENTED` | Launches official scrcpy with `--new-display` and `--start-app`; no physical device is available to validate the stream |
| Session lifecycle/error polling | `IMPLEMENTED` | Checks child process exit and retains a bounded stderr tail for failed sessions |
| Streaming presets | `IMPLEMENTED` | Eco/Balanced/Quality map to max size, bitrate, and FPS flags; preferences persist locally |
| Concurrent independent windows | `IN_PROGRESS` | One scrcpy process/window per app is the design; compatibility and independent interaction must be confirmed on physical hardware |
| Native window manager embedded in NESI shell | `PLANNED` | Current sessions are separate scrcpy-owned top-level Windows windows |
| Device OS/capability inventory | `PLANNED` | Current device list does not yet provide a full device capability profile |
| Bundled, signed ADB/scrcpy distribution | `PLANNED` | Dependencies must currently be installed separately |
| Windows installer smoke test | `PLANNED` | CI compiles/tests; packaging and installation still need explicit validation |
| Real-device verification | `BLOCKED` | No physical Android device was connected to this work environment |

## Next verification steps
1. Pass latest Windows frontend and Rust CI.
2. Run `npm run tauri build` and smoke-test the installer.
3. Connect at least one real Android device and complete `MANUAL_VALIDATION.md`.
4. Attempt two concurrent sessions and repeat on a second supported Android build.
5. Update status labels based on actual evidence, then promote the PR out of draft.
