import { invoke, isTauri } from "@tauri-apps/api/core";
import type { AndroidApp, AndroidDevice, Diagnostics, Session } from "../types";
const isDesktop=()=>isTauri();
export async function getDiagnostics():Promise<Diagnostics>{if(!isDesktop())return{adb:{available:false,message:"Open the desktop build to access ADB."},scrcpy:{available:false,message:"Streaming requires the desktop build."},devices:[]};return invoke<Diagnostics>("get_diagnostics");}
export async function refreshDevices():Promise<AndroidDevice[]>{if(!isDesktop())return[];return invoke<AndroidDevice[]>("list_devices");}
export async function listApps(deviceId:string):Promise<AndroidApp[]>{if(!isDesktop())throw new Error("App discovery is available in the NESI Desk desktop build.");return invoke<AndroidApp[]>("list_apps",{deviceId});}
export async function launchApp(deviceId:string,packageName:string,label:string,preset:"Eco"|"Balanced"|"Quality"):Promise<Session>{if(!isDesktop())throw new Error("Launch an Android app from the installed NESI Desk desktop application.");return invoke<Session>("launch_app",{deviceId,packageName,label,preset});}
export async function listSessions():Promise<Session[]>{if(!isDesktop())return[];return invoke<Session[]>("list_sessions");}
export async function pairWireless(address:string,pairingCode:string):Promise<string>{if(!isDesktop())throw new Error("Wireless ADB pairing is available in the NESI Desk desktop build.");return invoke<string>("pair_wireless",{address,pairingCode});}
export async function connectWireless(address:string):Promise<string>{if(!isDesktop())throw new Error("Wireless ADB is available in the NESI Desk desktop build.");return invoke<string>("connect_wireless",{address});}\nexport async function closeSession(sessionId:string):Promise<void>{if(!isDesktop())return;await invoke("close_session",{sessionId});}
