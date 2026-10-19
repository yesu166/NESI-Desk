use crate::android::{scrcpy_path, verify_device_authorized};
use serde::Serialize;
use std::{
    collections::{HashMap, VecDeque},
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub id: String,
    pub package_name: String,
    pub label: String,
    pub device_id: String,
    pub started_at: u64,
    pub state: String,
    pub error: Option<String>,
}

struct ManagedSession {
    info: SessionInfo,
    child: Child,
    stderr_lines: Arc<Mutex<VecDeque<String>>>,
}

#[derive(Default)]
pub struct SessionManager {
    sessions: HashMap<String, ManagedSession>,
}

impl SessionManager {
    pub fn launch(&mut self, device_id: &str, package_name: &str, label: &str) -> Result<SessionInfo, String> {
        validate_package(package_name)?;
        verify_device_authorized(device_id)?;
        let engine = scrcpy_path()?;
        let started_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let id = Uuid::new_v4().to_string();
        let title = format!("NESI Desk — {}", sanitize_title(label));
        let mut child = Command::new(engine)
            .args([
                "--serial", device_id,
                "--new-display=1280x960/160",
                "--start-app", package_name,
                "--window-title", &title,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Unable to start scrcpy: {e}"))?;

        let pipe = match child.stderr.take() {
            Some(pipe) => pipe,
            None => {
                let _ = child.kill();
                return Err("scrcpy started without an available diagnostics pipe.".into());
            }
        };
        let stderr_lines = Arc::new(Mutex::new(VecDeque::<String>::with_capacity(20)));
        let log_buffer = Arc::clone(&stderr_lines);
        thread::spawn(move || {
            for line in BufReader::new(pipe).lines() {
                let Ok(line) = line else { break };
                let cleaned: String = line.chars().filter(|c| !c.is_control()).take(500).collect();
                if let Ok(mut lines) = log_buffer.lock() {
                    if lines.len() >= 20 { lines.pop_front(); }
                    lines.push_back(cleaned);
                }
            }
        });

        let info = SessionInfo {
            id: id.clone(),
            package_name: package_name.to_string(),
            label: sanitize_title(label),
            device_id: device_id.to_string(),
            started_at,
            state: "starting".into(),
            error: None,
        };
        self.sessions.insert(id, ManagedSession {
            info: info.clone(),
            child,
            stderr_lines,
        });
        Ok(info)
    }

    /// Returns current process state. A live process is not proof that a video stream rendered,
    /// so the UI calls the healthy process state "process-active · stream unverified".
    pub fn list(&mut self) -> Vec<SessionInfo> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        for session in self.sessions.values_mut() {
            match session.child.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        session.info.state = "exited".into();
                        session.info.error = None;
                    } else {
                        session.info.state = "error".into();
                        let diagnostic = session.stderr_lines.lock().ok()
                            .map(|lines| lines.iter().cloned().collect::<Vec<_>>().join("\n"))
                            .filter(|text| !text.trim().is_empty());
                        session.info.error = Some(diagnostic.unwrap_or_else(|| format!("scrcpy exited with status {status}")));
                    }
                }
                Ok(None) => {
                    if session.info.state == "starting" && now.saturating_sub(session.info.started_at) >= 3 {
                        session.info.state = "process-active".into();
                    }
                }
                Err(error) => {
                    session.info.state = "error".into();
                    session.info.error = Some(format!("Could not inspect scrcpy process: {error}"));
                }
            }
        }
        self.sessions.values().map(|session| session.info.clone()).collect()
    }

    pub fn close(&mut self, id: &str) -> Result<(), String> {
        if let Some(mut session) = self.sessions.remove(id) {
            if session.child.try_wait().map_err(|e| format!("Could not inspect streaming process: {e}"))?.is_none() {
                session.child.kill().map_err(|e| format!("Could not stop streaming process: {e}"))?;
            }
            let _ = session.child.wait();
        }
        Ok(())
    }

    pub fn len(&self) -> usize { self.sessions.len() }
    pub fn is_empty(&self) -> bool { self.sessions.is_empty() }
}

impl Drop for SessionManager {
    fn drop(&mut self) {
        for (_, mut session) in self.sessions.drain() {
            if session.child.try_wait().ok().flatten().is_none() { let _ = session.child.kill(); }
            let _ = session.child.wait();
        }
    }
}

fn sanitize_title(label: &str) -> String {
    label.chars().filter(|c| !c.is_control()).take(80).collect()
}

fn validate_package(package: &str) -> Result<(), String> {
    let valid = !package.is_empty()
        && package.len() <= 255
        && package.split('.').all(|segment| {
            !segment.is_empty()
                && segment.chars().enumerate().all(|(i, c)| c.is_ascii_alphanumeric() || (i == 0 && c == '_'))
        });
    if valid { Ok(()) } else { Err("Invalid Android package name.".into()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_package() {
        assert!(validate_package("com.example.notes").is_ok());
        assert!(validate_package("com.google.android.youtube").is_ok());
        assert!(validate_package("_vendor.app").is_ok());
    }

    #[test]
    fn rejects_invalid_package() {
        assert!(validate_package("").is_err());
        assert!(validate_package("com.example; rm -rf /").is_err());
        assert!(validate_package("com..bad").is_err());
        assert!(validate_package("bad-name.pkg").is_err());
    }

    #[test]
    fn strips_control_chars_from_window_title() {
        assert_eq!(sanitize_title("A\nB"), "AB");
    }
}
