# NESI Desk

**Your phone computes. Your PC becomes the workspace.**

NESI Desk is a Windows-first desktop environment for launching real Android apps from a connected phone in separate desktop windows. Apps continue running on Android; the PC provides the desktop shell, display, keyboard, and mouse.

> **Status:** foundation in progress. ADB discovery, app listing, and scrcpy-backed session orchestration are being implemented. Independent multi-app sessions still require validation on physical Android devices.

## Prerequisites
- Windows 11 x64
- Node.js 20+ and npm
- Rust stable and Tauri 2 Windows prerequisites (WebView2 and C++ Build Tools)
- Android Platform Tools (`adb`)
- scrcpy v5.0.1 Windows x64
- Android phone with USB debugging enabled

## Development
```powershell
npm install
npm run tauri dev
```

Frontend checks: `npm run typecheck`, `npm run test`, `npm run build`.

Build installer: `npm run tauri build`.

ADB and scrcpy are not bundled yet. Add their executables to PATH or the local `tools/` directory.

## Connect
1. Enable Developer options and USB debugging on Android.
2. Connect using a data-capable USB cable.
3. Unlock the phone and approve the RSA authorization prompt.
4. Open NESI Desk and choose **Refresh devices**.
5. Select the authorized phone and refresh installed apps.
6. Launch an app. NESI Desk requests a scrcpy virtual display and reports failures rather than pretending success.

## Implementation status
| Capability | Status |
|---|---|
| React + TypeScript + Vite | Foundation |
| Tauri 2 + Rust boundary | Foundation |
| ADB detection and device discovery | Implemented |
| Preserve authorized / unauthorized / offline state | Implemented |
| Third-party package listing | Implemented |
| App launch on scrcpy virtual display | Orchestration implemented; hardware validation pending |
| Concurrent independent sessions | Process management implemented; hardware validation pending |
| Embedded native stream windows | Planned; current streams use scrcpy-owned windows |
| Wireless ADB | Planned |
| Bundled and checksum-verified tools | Planned |
| Phone telemetry | Planned; no fake stats shown |

## Troubleshooting
- **No device:** verify cable, USB debugging, and `adb devices -l`.
- **Unauthorized:** unlock the phone and approve the RSA prompt, then refresh.
- **ADB missing:** install Android Platform Tools and add `adb` to PATH.
- **scrcpy missing:** install the pinned Windows x64 release and add `scrcpy.exe` to PATH.
- **Virtual display rejected:** support varies by Android version and manufacturer. Review Diagnostics.
- **App not visible:** some apps restrict secondary displays or secure-window capture. NESI Desk does not bypass Android security.

## Engineering docs
- [Implementation plan](docs/IMPLEMENTATION_PLAN.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Manual device validation](docs/MANUAL_VALIDATION.md)
- [Third-party notices](THIRD_PARTY_NOTICES.md)

MIT for NESI Desk source. Third-party dependencies retain their own licenses.