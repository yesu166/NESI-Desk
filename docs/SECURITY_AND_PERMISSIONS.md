# Security and permissions

## Trust boundary
The React UI invokes a small set of Tauri commands. It cannot submit arbitrary shell commands. Rust validates inputs and invokes `adb` and `scrcpy` with argument arrays.

## Device authorization
- A device is launch-ready only when ADB reports state `device`.
- `unauthorized`, `offline`, and disconnected states are not treated as successful connections.
- USB debugging authorization must be approved on the phone by the user.
- Wireless pairing requires the user to enable Android Wireless debugging and enter its temporary pairing details.

## Input validation
- Package identifiers are checked before launch.
- Wireless pairing/connect addresses must parse as IP:port socket addresses.
- Pairing codes are restricted to digits and a short length; they are passed to ADB only for the pairing command and are not saved in preferences.
- Window titles strip control characters.
- User-provided strings are not interpolated into a shell command.

## Local data and network behavior
There is no cloud backend, account system, telemetry service, or analytics endpoint. The web font import was removed to avoid a required remote font request. Theme and streaming preset are stored in local webview preferences.

## Process lifecycle
- Streaming child processes are owned by the Rust session manager.
- Session status polling checks for early process exit.
- On NESI Desk shutdown, the manager attempts to terminate child processes.
- The last bounded stderr lines are retained in memory for an engine failure; pairing codes are never logged.

## Known gaps
- A process still running does not prove its stream is rendering; the UI explicitly labels this state unverified.
- Native window handle/focus orchestration is not yet implemented inside NESI Desk.
- ADB/scrcpy are external dependencies and are not checksum-verified or bundled automatically.
- Physical Android security behaviors and multi-session compatibility have not been validated here.
