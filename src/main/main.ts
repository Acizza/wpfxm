import { app } from "electron";
import "./ipc/prefix/prefix";
import { createMainWindow } from "./window";

app.whenReady().then(createMainWindow);
