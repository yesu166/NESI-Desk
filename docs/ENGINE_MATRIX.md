# Engine matrix

| Component | Initial selection | Origin / licensing | Packaging | Verification |
|---|---|---|---|---|
| Desktop host | Tauri 2 | Open-source; see upstream terms | Built by Tauri | Rust/Windows CI; installer smoke test pending |
| UI | React + TypeScript + Vite | Open-source dependencies; inventory with npm tooling | Bundled into Tauri webview assets | Typecheck, Vitest, and Vite build |
| Device transport | Android Platform Tools / ADB | Official Android platform-tools distribution | External prerequisite for now; PATH or `./tools` | Tool detection implemented |
| App streaming | scrcpy v5.0.1 (Windows x64 intended) | Official scrcpy distribution; Apache License 2.0 | External prerequisite for now; PATH or `./tools` | Process integration implemented; physical-device validation pending |

## Distribution gate
Before shipping, record exact ADB/scrcpy archive versions and SHA-256 checksums, include required license notices, choose an update strategy, and verify that bundling complies with the upstream distribution terms. Do not download or execute an unknown binary automatically.
