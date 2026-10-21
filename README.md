# NESI Desk

**The phone computes. The PC provides the desktop.**

NESI Desk is a Windows-first, local-first workspace for launching real Android applications from a connected phone. Android apps keep running on the phone; the PC provides the monitor, keyboard, mouse, and native desktop windows created by scrcpy.

> **Release status: in progress.** The desktop shell, ADB discovery, package listing, wireless ADB commands, session process manager, session error polling, and streaming presets are implemented in code. A physical Android device is still required to verify real streams and concurrent independent sessions.

## Prerequisites

- Windows 11 x64
- Node.js 20+ and npm
- Rust stable
- Tauri 2 Windows prerequisites: Microsoft C++ Build Tools and WebView2
- Android Platform Tools (`adb`)
- scrcpy v5.0.1 for Windows x64
- An Android phone with Developer options enabled

ADB and scrcpy are not bundled in this development build. Add `adb.exe` and `scrcpy.exe` to PATH or put them under `./tools/`.

## Run locally

```powershell
npm install
npm run tauri dev
```

Frontend verification:

```powershell
npm run typecheck
npm run test
npm run build
```

Rust tests:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml
```

Build a Windows installer:

```powershell
npm run tauri build
```

## Connect over USB

1. On Android, enable Developer options → USB debugging.
2. Connect with a data-capable USB cable.
3. Unlock the phone and approve the RSA authorization prompt.
4. Open NESI Desk and choose **Refresh devices**.
5. Select the device whose ADB state is **Authorized**.
6. Refresh installed apps and launch an app.

NESI Desk requests a new virtual display and asks scrcpy to start the selected package. A process launch alone is not considered proof that video is rendering; Diagnostics and session status distinguish process state from verified streaming.

## Connect wirelessly (optional)

1. Put the PC and phone on the same trusted Wi-Fi network.
2. On Android 11+, open Developer options → Wireless debugging.
3. Choose **Pair device with pairing code** and enter the displayed pairing address and temporary code in the NESI Desk **Devices** screen.
4. Use the separate IP address and port from the main Wireless debugging screen in **Connection address**, then choose **Connect wirelessly**.
5. Refresh devices and select the authorized wireless device.

The pairing port and connection port are often different. The pairing code is sent directly to ADB, is not saved in preferences, and should only be entered on a trusted PC/network.

## Streaming presets

- **Eco:** 1024 px maximum size, 2 Mbps target bitrate, 30 FPS cap.
- **Balanced:** 1600 px maximum size, 8 Mbps target bitrate, 60 FPS cap.
- **Quality:** 1920 px maximum size, 16 Mbps target bitrate, 60 FPS cap.

These are requested scrcpy limits, not guaranteed performance measurements. Actual output depends on device support, network/USB transport, available encoders, and the target app. The selected preset and theme are stored locally.

## Implementation status

| Capability | Status |
|---|---|
| React + TypeScript + Vite desktop UI | Implemented; checked in Windows CI |
| Tauri 2 + Rust command boundary | Implemented; Rust CI is required on the latest revision |
| ADB executable detection and device states | Implemented |
| Third-party package discovery | Implemented; labels/icons currently derive from package IDs |
| USB authorization guidance | Implemented |
| Wireless ADB pairing and connect | Implemented in code; physical-device verification pending |
| Eco/Balanced/Quality stream flags | Implemented in code |
| Session exit detection and bounded stderr diagnostics | Implemented in code |
| One app per scrcpy virtual display | Process orchestration implemented; hardware verification pending |
| Concurrent independent app sessions | Not verified on physical hardware |
| NESI-managed embedded video windows | Planned; streams currently use separate scrcpy-owned Windows windows |
| Device capability inventory and real performance telemetry | Planned |
| Bundled, checksum-verified ADB/scrcpy dependencies | Planned |
| Packaged installer smoke test and code signing | Planned |

## Troubleshooting

- **No device:** check cable/data mode, USB debugging, and `adb devices -l`.
- **Unauthorized:** unlock the phone and approve the RSA prompt.
- **ADB missing:** install official Android Platform Tools and add `adb.exe` to PATH.
- **scrcpy missing:** install the pinned Windows x64 release and add `scrcpy.exe` to PATH.
- **Wireless pair fails:** confirm the phone is on Wireless debugging's pairing screen and that the pairing address/code are current.
- **Wireless connect fails:** use the separate connection port shown on Wireless debugging, not necessarily the pairing port.
- **Virtual display rejected:** support varies by Android version/manufacturer and app. Read the exact session error.
- **App not visible:** some apps restrict secondary displays or secure-window capture. NESI Desk does not bypass Android security.
- **Session says “process active · stream unverified”:** the scrcpy process is alive, but only manual/visual validation can confirm the stream is usable.

## Engineering documents

- [Implementation plan](docs/IMPLEMENTATION_PLAN.md)
- [Implementation status](docs/IMPLEMENTATION_STATUS.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Manual device validation](docs/MANUAL_VALIDATION.md)
- [Engine matrix](docs/ENGINE_MATRIX.md)
- [Security and permissions](docs/SECURITY_AND_PERMISSIONS.md)
- [Release checklist](docs/RELEASE_CHECKLIST.md)
- [Third-party notices](THIRD_PARTY_NOTICES.md)

NESI Desk's source repository carries the existing MIT license. Third-party components retain their own licenses.

## Project brief

For the project's purpose, scope, current maturity, proposed roadmap, limitations, and collaboration opportunities, see [PROJECT_BRIEF.md](PROJECT_BRIEF.md).
