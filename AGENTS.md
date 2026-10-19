# NESI Desk contributor notes

## Product boundary
NESI Desk runs applications on a real Android device. Do not add an emulator or claim a separate application window works unless it is backed by a real Android virtual-display session. Current stream windows are created and owned by scrcpy; they are not embedded in the NESI Desk shell.

## Stack
- Windows-first desktop shell: Tauri 2 + Rust.
- UI: React + TypeScript + Vite.
- Device control: ADB.
- Initial streaming engine: scrcpy v5.0.1, discovered from PATH or `./tools`.

## Commands
- `npm install`
- `npm run typecheck`
- `npm run test`
- `npm run build`
- `npm run tauri dev`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `npm run tauri build`

## Safety and correctness
- Do not expose arbitrary shell execution to React.
- Execute native tools in Rust with discrete argument arrays, not shell-concatenated strings.
- Validate all device addresses and package names before passing them to native tools.
- Never treat a process spawn as proof that a video stream is live.
- Keep unauthorized/offline ADB states distinct from authorized `device`.
- Do not log or persist wireless pairing codes.
- Don't check third-party ADB/scrcpy binaries into this repository; verify release archives, checksums, and licenses before bundling.
- Do not claim physical-device or multi-session verification without running the manual validation procedure.
