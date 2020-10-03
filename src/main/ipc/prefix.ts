import { ipcMain } from "electron";
import * as fs from "fs";
import * as path from "path";
import { IPC } from "../../shared/ipc/event";
import { IPrefix } from "../../shared/ipc/prefix";
import { normalizeUnixPath } from "../util";

export async function allFromDir(dir: string): Promise<IPrefix[]> {
  if (!dir.length) return [];

  const absPath = normalizeUnixPath(dir);
  const files = await fs.promises.readdir(absPath);
  const results = [];

  for (const file of files) {
    const pfxPath = path.join(absPath, file);
    const type = await fs.promises.stat(pfxPath);

    if (!type.isDirectory() || !isValidPrefix(pfxPath)) continue;

    const prefix = {
      name: file,
      path: pfxPath,
    };

    results.push(prefix);
  }

  return results;
}

function isValidPrefix(_: string): boolean {
  // TODO
  return true;
}

ipcMain.handle(IPC.ScanPrefixes, (_, path: string) => allFromDir(path));

export default {};
