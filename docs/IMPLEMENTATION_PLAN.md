# Implementation plan

## Baseline
The default branch initially contained only `LICENSE`; no source tree or tests existed. Changes are staged on `feat/nesi-desk-foundation`, leaving `main` untouched.

## Milestones
### M1 — Desktop foundation
- [x] Vite + React + TypeScript foundation planned
- [x] Tauri 2 + Rust boundary planned
- [x] ADB/scrcpy diagnostics architecture
- [ ] Validate typecheck/build on Windows

### M2 — Device connectivity
- [x] ADB device discovery and actual states designed
- [x] Preserve unauthorized/offline/device statuses
- [ ] App display-name/icon extraction and wireless ADB pairing

### M3 — App sessions
- [x] scrcpy virtual-display process strategy documented
- [ ] Detect early process exits and expose stderr
- [ ] Real-device concurrent session validation
- [ ] Embed streams in NESI-managed native windows

### M4 — Reliability and distribution
- [ ] Bundle/checksum-verify ADB and scrcpy
- [ ] Process-runner integration tests
- [ ] Windows CI and installer smoke test

### M5 — Release polish
- [ ] Persist preferences, accessibility audit, diagnostic export, signing
- [ ] Validate at least two Android device/OS combinations

## Acceptance criteria
- Browser preview never fabricates a phone.
- Device states come from `adb devices -l`.
- Missing tools yield actionable errors.
- Arguments are passed as discrete process arguments, not shell concatenation.
- Child sessions are terminated during shutdown.
- Multi-app support is not called verified until independent sessions pass on real hardware.

## Primary risk
Virtual-display and multi-encoder support depends on Android release, OEM, device resources, and target application. Spawning a process does not prove streaming succeeds.