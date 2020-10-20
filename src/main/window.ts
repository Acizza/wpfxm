import { BrowserWindow } from "electron";
import * as path from "path";
import { config } from "../../package.json";

let globalWindow: BrowserWindow;

export function createMainWindow(): void {
  globalWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: false,
      enableRemoteModule: false,
      sandbox: true,
      preload: path.join(__dirname, "electron_preload.js"),
    },
  });

  globalWindow.setMenuBarVisibility(false);

  if (process.env.NODE_ENV === "development") {
    globalWindow.loadURL(`http://localhost:${config.dev_port}`);
  } else {
    globalWindow.loadFile("public/index.html");
  }
}

export function mainWindow(): BrowserWindow {
  return globalWindow;
}
