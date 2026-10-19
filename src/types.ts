export type DeviceState = "device" | "unauthorized" | "offline" | "unknown";
export interface AndroidDevice { id:string; state:DeviceState; model?:string; product?:string; transportId?:string; }
export interface AndroidApp { packageName:string; label:string; }
export interface ToolStatus { available:boolean; path?:string; version?:string; message:string; }
export interface Diagnostics { adb:ToolStatus; scrcpy:ToolStatus; devices:AndroidDevice[]; }
export interface Session { id:string; packageName:string; label:string; deviceId:string; startedAt:number; state:"starting"|"running"|"stopping"|"error"; error?:string; }
