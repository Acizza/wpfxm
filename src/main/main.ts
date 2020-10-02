import { app, BrowserWindow } from "electron";

function createWindow(): void {
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true,
    },
  });

  win.setMenuBarVisibility(false);

  if (process.env.NODE_ENV === "development") {
    win.loadURL(
      `http://localhost:${process.env.npm_package_config_dev_port || 3000}`
    );
  } else {
    win.loadFile("public/index.html");
  }
}

app.whenReady().then(createWindow);
