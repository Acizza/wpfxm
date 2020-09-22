const { app, BrowserWindow } = require('electron');

function createWindow() {
    // Create the browser window.
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true
        }
    });

    win.setMenuBarVisibility(false);

    if (process.env.NODE_ENV === "development") {
        win.loadURL(`http://localhost:${process.env.npm_package_config_dev_port || 3000}`);
    } else {
        win.loadFile("build/index.html");
    }
}

app.whenReady().then(createWindow);