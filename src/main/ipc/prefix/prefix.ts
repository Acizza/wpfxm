import { ipcMain } from "electron";
import * as fs from "fs";
import * as path from "path";
import { IPC } from "../../../shared/ipc/event";
import { Prefix } from "../../../shared/ipc/prefix";
import { NormalizedPath } from "../../util";
import "./application";

async function allFromDir(dir: NormalizedPath): Promise<Prefix[]> {
  if (!dir.length) return [];

  const files = await fs.promises.readdir(dir.path);
  const results = [];

  for (const file of files) {
    const pfxPath = path.join(dir.path, file);
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

function isValidPrefix(pfxPath: string): boolean {
  const systemRegPath = path.join(pfxPath, "system.reg");

  if (!fs.existsSync(systemRegPath)) return false;

  const cDrivePath = path.join(pfxPath, "drive_c");

  if (!fs.statSync(cDrivePath).isDirectory()) return false;

  return true;
}

ipcMain.handle(IPC.ScanPrefixes, (_, path: string) =>
  allFromDir(new NormalizedPath(path))
);

export default {};
