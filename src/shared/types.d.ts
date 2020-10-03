import { IpcRenderer } from "electron";

export declare global {
  interface Window {
    ipc: IpcRenderer;
  }
}
