mod android;
mod sessions;
use android::{discover_devices,diagnostics_for,installed_apps,AppInfo,DeviceInfo,Diagnostics};
use sessions::{SessionManager,SessionInfo};
use std::sync::Mutex;
use tauri::State;
pub struct AppState{pub sessions:Mutex<SessionManager>}
#[tauri::command]fn get_diagnostics()->Diagnostics{diagnostics_for()}
#[tauri::command]fn list_devices()->Result<Vec<DeviceInfo>,String>{discover_devices()}
#[tauri::command]fn list_apps(device_id:String)->Result<Vec<AppInfo>,String>{installed_apps(&device_id)}
#[tauri::command]fn launch_app(device_id:String,package_name:String,label:String,state:State<'_,AppState>)->Result<SessionInfo,String>{let mut m=state.sessions.lock().map_err(|_|"Session manager lock was poisoned".to_string())?;m.launch(&device_id,&package_name,&label)}
#[tauri::command]fn close_session(session_id:String,state:State<'_,AppState>)->Result<(),String>{let mut m=state.sessions.lock().map_err(|_|"Session manager lock was poisoned".to_string())?;m.close(&session_id)}
#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){tauri::Builder::default().manage(AppState{sessions:Mutex::new(SessionManager::default())}).invoke_handler(tauri::generate_handler![get_diagnostics,list_devices,list_apps,launch_app,close_session]).build(tauri::generate_context!()).expect("error while building NESI Desk").run(|_,event|{if let tauri::RunEvent::Exit=event{/* SessionManager drop cleans child processes. */}});}
