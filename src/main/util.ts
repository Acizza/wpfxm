import * as path from "path";
import * as os from "os";
import * as fs from "fs";
import { ipcMain } from "electron";
import { IPCSync } from "../shared/ipc/event";

export function normalizeUnixPath(value: string): string {
  if (!value.length) return value;

  const slice =
    value[0] === "~" ? path.join(os.homedir(), value.slice(1)) : value;

  return path.normalize(slice);
}

export class NormalizedPath {
  #path: string;

  constructor(path: string) {
    this.#path = normalizeUnixPath(path);
  }

  get path(): string {
    return this.#path;
  }

  set path(value: string) {
    this.#path = normalizeUnixPath(value);
  }

  get length(): number {
    return this.path.length;
  }
}

ipcMain.on(IPCSync.NormalizePath, (event, path: string) => {
  event.returnValue = normalizeUnixPath(path);
});

ipcMain.on(IPCSync.FileExists, (event, path: string) => {
  event.returnValue = fs.existsSync(path);
});
