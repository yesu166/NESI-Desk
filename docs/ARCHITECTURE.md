# Architecture

React + TypeScript (Vite) calls narrow Tauri commands. Rust owns process execution and validation. ADB handles device discovery, authorization and package inventory. scrcpy v5.0.1 handles streaming, input forwarding, virtual-display creation and host rendering.

No local web server, cloud account, analytics service or database is required.

## Session invocation
Each app session uses arguments equivalent to:

```powershell
scrcpy --serial <device-id> --new-display=1280x960/160 --start-app <package-name> --window-title "NESI Desk — <app-name>"
```

Arguments are passed separately instead of through a shell. The ADB device must be authorized, and Android package syntax is validated.

The scrcpy virtual-display documentation provides `--new-display` and `--start-app`; device support and concurrent encoders still require hardware validation. A spawned process is not proof of a live stream.

Separate scrcpy processes are a pragmatic prototype and reuse upstream transport/input logic. Their windows are currently scrcpy-owned, not embedded in NESI's shell; embedding is a later milestone.

## Safety
- No general shell endpoint is exposed to React.
- Process execution uses argument arrays and detached stdin.
- ADB-reported state is not replaced by mock data.
- Host tests do not claim physical-device compatibility.