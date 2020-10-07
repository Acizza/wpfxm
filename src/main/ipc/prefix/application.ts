import { IPrefix } from "../../../shared/ipc/prefix";
import { NormalizedPath } from "../../util";
import * as fs from "fs";
import * as path from "path";
import { ipcMain } from "electron";
import { IPC } from "../../../shared/ipc/event";
import {
  FoundApplications,
  ApplicationPath,
} from "../../../shared/ipc/application";

const applicationExt = ".exe";

async function scanApplications(prefix: IPrefix): Promise<FoundApplications> {
  const pfxDir = new NormalizedPath(prefix.path);
  let { execs, dirs } = await filesWithExtension(pfxDir.path, applicationExt);

  while (dirs.length > 0) {
    const dir = dirs.pop() as string;
    const found = await filesWithExtension(dir, applicationExt);

    found.execs.forEach((exec) => execs.push(exec));
    found.dirs.forEach((dir) => dirs.push(dir));
  }

  const commonPathPrefix = findCommonPathPrefix(execs);

  const paths: ApplicationPath[] = execs.map((exec) => {
    // Our stripped path should nclude the path separator so it doesn't start with /
    const start = commonPathPrefix.length + 1;
    const end = exec.length - applicationExt.length;
    const stripped = exec.slice(start, end);

    return { absolute: exec, stripped };
  });

  return {
    paths,
    commonPathPrefix,
  };
}

ipcMain.handle(IPC.ScanPrefixApps, (_, prefix: IPrefix) =>
  scanApplications(prefix)
);

function findCommonPathPrefix(paths: string[]): string {
  if (paths.length === 0) return "";

  let prefix = path.dirname(paths[0]);
  let fragments = prefix.split(path.sep);

  for (const pfxPath of paths) {
    while (fragments.length > 0) {
      if (pfxPath.startsWith(prefix)) break;

      fragments.pop();
      prefix = fragments.join(path.sep);
    }
  }

  return prefix;
}

interface FoundFiles {
  execs: string[];
  dirs: string[];
}

const excludedFolders = [
  "windows",
  "windows nt",
  "windows media player",
  "internet explorer",
];

async function filesWithExtension(
  dir: string,
  extension: string
): Promise<FoundFiles> {
  const entries = await fs.promises.readdir(dir, { withFileTypes: true });

  const execs = [];
  const dirs = [];

  for (const entry of entries) {
    const absPath = path.join(dir, entry.name);

    if (entry.isDirectory()) {
      if (arrContainsIgnoreCase(excludedFolders, entry.name)) continue;
      dirs.push(absPath);
    }

    const fileExt = path.extname(entry.name);

    if (!eqIgnoreCase(fileExt, extension)) continue;

    execs.push(absPath);
  }

  return {
    execs,
    dirs,
  };
}

function eqIgnoreCase(x: string, y: string): boolean {
  return x.localeCompare(y, undefined, { sensitivity: "base" }) === 0;
}

function arrContainsIgnoreCase(arr: string[], value: string): boolean {
  return arr.some((x) => eqIgnoreCase(x, value));
}

export default {};
