import { Prefix, PrefixArch } from "../../../shared/ipc/prefix";
import { NormalizedPath } from "../../util";
import * as fs from "fs";
import * as path from "path";
import * as process from "process";
import * as os from "os";
import { ipcMain } from "electron";
import { IPC, IPCSync } from "../../../shared/ipc/event";
import {
  FoundApplications,
  ApplicationPath,
  LaunchOptions,
  AppEvent,
  maxAppEvents,
} from "../../../shared/ipc/application";
import { ChildProcess, spawn, SpawnOptionsWithoutStdio } from "child_process";
import { mainWindow } from "../../window";

const applicationExt = ".exe";

async function scanApplications(prefix: Prefix): Promise<FoundApplications> {
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

ipcMain.handle(IPC.ScanPrefixApps, (_, prefix: Prefix) =>
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

interface TrackedApp {
  process: ChildProcess;
  events: AppEvent[];
}

type AbsolutePath = string;

const runningApps: Map<AbsolutePath, TrackedApp> = new Map();

function launch(opts: LaunchOptions): void {
  const is32Bit = opts.force32Bit || opts.app.prefix.arch === PrefixArch.X32;
  const wineExec = is32Bit ? "wine" : "wine64";

  const spawnOpts: SpawnOptionsWithoutStdio = {
    env: {
      WINEPREFIX: opts.app.prefix.path,
      WINEARCH: is32Bit ? "win32" : "win64",
      ...process.env,
      ...opts.env,
    },
    stdio: "pipe",
    detached: true,
    windowsHide: true,
  };

  const absPath = opts.app.path.absolute;
  const pc = spawn(wineExec, [absPath, ...(opts.args || [])], spawnOpts);

  runningApps.set(absPath, {
    process: pc,
    events: runningApps.get(absPath)?.events || [],
  });

  function onData(data: Buffer | string) {
    const runningApp = runningApps.get(absPath);

    data
      .toString()
      .split(os.EOL)
      .filter((line) => line.length > 0)
      .map((line) => ({ kind: "out", data: line } as AppEvent))
      .forEach((msg) => {
        if (runningApp) {
          const output = runningApp.events;
          const slice =
            output.length > maxAppEvents
              ? output.slice(output.length - maxAppEvents)
              : output;

          slice.push(msg);
        }
        mainWindow().webContents.send(IPC.AppEvent, absPath, msg);
      });
  }

  pc.stdout.on("data", onData);
  pc.stderr.on("data", onData);

  pc.on("close", () => {
    const reply: AppEvent = {
      kind: "close",
      app: opts.app,
    };

    const runningApp = runningApps.get(absPath);
    if (runningApp) runningApp.events.push(reply);

    mainWindow().webContents.send(IPC.AppEvent, absPath, reply);
  });

  mainWindow().webContents.send(IPC.AppEvent, absPath, {
    kind: "launch",
    app: opts.app,
  } as AppEvent);
}

ipcMain.handle(IPC.LaunchApp, (_, opts: LaunchOptions) => launch(opts));

ipcMain.handle(IPC.CloseApp, (_, absPath: string) => {
  const running = runningApps.get(absPath);
  if (!running) return;

  if (!running.process.kill()) return false;

  return new Promise((resolve) =>
    running.process.on("close", (code) => {
      const success = code === null || code === 0;
      resolve(success);
    })
  );
});

ipcMain.on(IPCSync.GetAppEvents, (event, absPath: string) => {
  const output: AppEvent[] | undefined = runningApps.get(absPath)?.events;
  event.returnValue = output;
});

export default {};
