use crate::android::{scrcpy_path, verify_device_authorized};
use serde::Serialize;
use std::{collections::HashMap, process::{Child, Command, Stdio}, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo { pub id:String, pub package_name:String, pub label:String, pub device_id:String, pub started_at:u64, pub state:String, pub error:Option<String> }
struct ManagedSession { child:Child }
#[derive(Default)]
pub struct SessionManager { sessions:HashMap<String,ManagedSession> }
impl SessionManager {
 pub fn launch(&mut self, device_id:&str, package_name:&str, label:&str)->Result<SessionInfo,String>{
  verify_device_authorized(device_id)?; validate_package(package_name)?; let engine=scrcpy_path()?;
  let started_at=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(); let id=Uuid::new_v4().to_string();
  let title=format!("NESI Desk — {}",sanitize_title(label));
  let child=Command::new(engine).args(["--serial",device_id,"--new-display=1280x960/160","--start-app",package_name,"--window-title",&title]).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().map_err(|e|format!("Unable to start scrcpy: {e}"))?;
  self.sessions.insert(id.clone(),ManagedSession{child});
  Ok(SessionInfo{id,package_name:package_name.into(),label:label.into(),device_id:device_id.into(),started_at,state:"starting".into(),error:None})
 }
 pub fn close(&mut self,id:&str)->Result<(),String>{if let Some(mut s)=self.sessions.remove(id){if s.child.try_wait().map_err(|e|format!("Could not inspect streaming process: {e}"))?.is_none(){s.child.kill().map_err(|e|format!("Could not stop streaming process: {e}"))?;}let _=s.child.wait();}Ok(())}
 pub fn len(&self)->usize{self.sessions.len()} pub fn is_empty(&self)->bool{self.sessions.is_empty()}
}
impl Drop for SessionManager { fn drop(&mut self){for(_,mut s)in self.sessions.drain(){if s.child.try_wait().ok().flatten().is_none(){let _=s.child.kill();}let _=s.child.wait();}} }
fn sanitize_title(s:&str)->String{s.chars().filter(|c|!c.is_control()).take(80).collect()}
fn validate_package(p:&str)->Result<(),String>{let valid=!p.is_empty()&&p.len()<=255&&p.split('.').all(|s|!s.is_empty()&&s.chars().enumerate().all(|(i,c)|c.is_ascii_alphanumeric()||(i==0&&c=='_')));if valid{Ok(())}else{Err("Invalid Android package name.".into())}}
#[cfg(test)]mod tests{use super::*;#[test]fn validates_package(){assert!(validate_package("com.example.notes").is_ok());assert!(validate_package("com.google.android.youtube").is_ok());assert!(validate_package("_vendor.app").is_ok());}#[test]fn rejects_invalid_package(){assert!(validate_package("").is_err());assert!(validate_package("com.example; rm -rf /").is_err());assert!(validate_package("com..bad").is_err());assert!(validate_package("bad-name.pkg").is_err());}#[test]fn strips_control_chars(){assert_eq!(sanitize_title("A\nB"),"AB");}}
