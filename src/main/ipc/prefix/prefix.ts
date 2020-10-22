import { ipcMain } from "electron";
import * as fs from "fs";
import * as path from "path";
import * as readline from "readline";
import { IPC } from "../../../shared/ipc/event";
import { Prefix, PrefixArch } from "../../../shared/ipc/prefix";
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

    const arch = await detectArch(pfxPath);

    if (arch === null) continue;

    const prefix = {
      name: file,
      path: pfxPath,
      arch,
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

async function detectArch(pfxPath: string): Promise<PrefixArch | null> {
  async function fromPath(filePath: string): Promise<PrefixArch | null> {
    const stream = fs.createReadStream(filePath);
    const lines = readline.createInterface(stream);

    for await (const line of lines) {
      const separator = line.split("#arch=", 2);

      if (separator.length < 2) continue;

      switch (separator[1]) {
        case "win32":
          return PrefixArch.X32;
        case "win64":
          return PrefixArch.X64;
      }
    }

    lines.close();
    stream.close();

    return null;
  }

  // These files contain the prefix architecture, and are sorted in increasing order of
  // size incase one file doesn't have the line we're looking for
  const files = ["userdef.reg", "user.reg", "system.reg"];

  for (const file of files) {
    const fullPath = path.join(pfxPath, file);
    const arch = fromPath(fullPath);

    if (arch) return arch;
  }

  return null;
}

ipcMain.handle(IPC.ScanPrefixes, (_, path: string) =>
  allFromDir(new NormalizedPath(path))
);

export default {};
