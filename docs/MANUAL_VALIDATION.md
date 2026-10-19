# Manual Android validation

## Prepare
1. Use Windows 11 x64.
2. Install Android Platform Tools and scrcpy v5.0.1 x64.
3. Record SHA-256 hashes and keep dependency license notices.
4. Add `adb.exe` and `scrcpy.exe` to PATH or `./tools/`.
5. Enable Developer options → USB debugging, connect the phone, unlock it, and accept the RSA prompt.
6. Check `adb devices -l` shows the serial as `device`.

## Device/package checks
1. Refresh devices in NESI Desk and compare serial/model with ADB output.
2. Test authorized, unauthorized, offline and disconnected states.
3. Compare package results with `adb -s <serial> shell pm list packages -3`.

## One app
1. Launch a non-sensitive test app such as Android Settings.
2. Confirm a separate scrcpy window shows the selected app on a virtual display.
3. Test click, drag, scroll, keyboard, resize and close.
4. Check the process exits and virtual display is removed.

## Concurrent sessions
1. Launch two different supported apps.
2. Confirm separate windows show separate apps rather than cropped portions of one screen.
3. Test input targeting and close one session while the other remains usable.
4. Record model, Android build, package names, engine version, logs, and resource/thermal observations.
5. Repeat with another Android version or manufacturer.

Some devices/apps reject virtual displays, secondary displays, secure-window capture, or concurrent encoders. The correct result is an explicit failure, never a simulated success.