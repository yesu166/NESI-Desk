# NESI Desk | Project Brief

> **Proposed repository description:** Windows desktop workspace for launching Android apps from a connected phone using ADB and scrcpy, with device discovery, session management, and streaming presets.

## Overview
NESI Desk is a Windows-first desktop workspace that uses a connected Android phone to run Android applications while the PC provides the monitor, keyboard, mouse, and separate scrcpy-owned windows.

## Problem and opportunity
Developers and power users may want to interact with real Android apps from a desktop without pretending that the PC is running those apps natively. A useful tool should make device connection, permissions, stream state, and errors visible instead of hiding them behind a generic launch button.

## Proposed approach
- Use React, TypeScript, Vite, Tauri 2, and Rust for the desktop application.
- Discover Android devices through ADB and support USB plus optional wireless debugging.
- Use scrcpy to request and manage app sessions and configurable stream presets.
- Separate process state from verified stream state.
- Provide explicit connection and authorization guidance and bounded session diagnostics.

## Current scope and maturity
The README reports that the desktop shell, ADB discovery, package listing, wireless ADB commands, session process management, error polling, and streaming presets are implemented in code. It also states that physical-device verification is still needed for real streams and concurrent independent sessions, and that embedded NESI-managed video windows are planned rather than implemented.

## Roadmap proposal
1. Run the manual validation checklist on a documented set of Android devices and Windows systems.
2. Test USB authorization, wireless pairing, connection recovery, process exit, and user cancellation.
3. Distinguish app launch, process health, and visually verified streaming throughout the UI.
4. Validate concurrent sessions and document Android/device limitations.
5. Package dependencies safely with version pinning and checksum verification where distribution terms allow.
6. Complete installer smoke tests, release notes, and a clear troubleshooting guide.

## Security and privacy boundaries
ADB debugging grants powerful access to a device. Users should enable it only for trusted computers and networks and disable it when no longer needed. NESI Desk should not bypass Android application security restrictions or imply that every app supports secondary displays or screen capture.

## Collaboration proposal
Useful contributions include device compatibility reports, reproducible bug reports, accessibility, installer checks, session lifecycle tests, and documentation. Include Android version, device, connection method, and logs with sensitive values removed.

## Licensing and reuse
The README states that the source repository carries an MIT license; third-party components retain their own licences. Review scrcpy, Android Platform Tools, and bundled dependency terms before redistributing a packaged build.
