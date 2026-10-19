import { describe,expect,it } from "vitest";
import { getDiagnostics,refreshDevices } from "./bridge";
describe("browser-safe bridge",()=>{it("does not invent a connected device in browser preview",async()=>{expect(await refreshDevices()).toEqual([]);});it("clearly explains device access needs desktop runtime",async()=>{const d=await getDiagnostics();expect(d.adb.available).toBe(false);expect(d.adb.message).toContain("desktop build");expect(d.devices).toEqual([]);});});
