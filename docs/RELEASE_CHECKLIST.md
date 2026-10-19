# Release checklist

## Source and build
- [ ] Latest Windows CI passes frontend typecheck, Vitest, Vite build, and Rust tests.
- [ ] `npm run tauri build` creates a Windows installer from a clean checkout.
- [ ] Install and launch the packaged app on a clean Windows 11 x64 machine.
- [ ] Confirm the bundled application icon is valid in the installer and executable.
- [ ] Run dependency/license inventory and record shipped versions.

## Device integration
- [ ] Missing ADB and scrcpy each produce actionable messages.
- [ ] Test authorized, unauthorized, offline, and disconnected ADB states.
- [ ] Pair/connect over Wireless debugging with a real device.
- [ ] Launch one real app and verify input, resize, window close, and display cleanup.
- [ ] Launch two different apps; verify truly independent streams/windows.
- [ ] Close one session while the other remains functional.
- [ ] Repeat with a second Android version or manufacturer.

## Privacy and safety
- [ ] Verify no network requests are required for routine operation.
- [ ] Verify pairing codes are not written to logs or preferences.
- [ ] Review all process invocation sites for shell-string construction.
- [ ] Confirm child processes are terminated on application shutdown.

## Distribution
- [ ] Pin and checksum ADB/scrcpy releases.
- [ ] Bundle tools only after license/distribution review.
- [ ] Add signing and SmartScreen/release guidance as appropriate.
- [ ] Publish accurate feature matrix and known limitations.
- [ ] Do not call concurrent app support verified until real-device tests pass.
