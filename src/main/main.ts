import { app, BrowserWindow } from "electron";
import * as path from "path";
import { config } from "../../package.json";
import "./ipc/prefix/prefix";

function createWindow(): void {
  const win = new BrowserWindow({
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

  win.setMenuBarVisibility(false);

  if (process.env.NODE_ENV === "development") {
    win.loadURL(`http://localhost:${config.dev_port}`);
  } else {
    win.loadFile("public/index.html");
  }
}

app.whenReady().then(createWindow);
