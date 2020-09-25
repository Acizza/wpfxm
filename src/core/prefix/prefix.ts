import * as fs from "fs";
import * as path from "path";
import { normalizeUnixPath } from "../util";

class Prefix {
  #name: string;
  #path: string;

  constructor(name: string, path: string) {
    this.#name = name;
    this.#path = path;
  }

  get name() {
    return this.#name;
  }

  get path() {
    return this.#path;
  }

  static async allFromDir(dir: string): Promise<Prefix[]> {
    if (!dir.length) return [];

    const absPath = normalizeUnixPath(dir);
    const files = await fs.promises.readdir(absPath);
    const results: Prefix[] = [];

    for (const file of files) {
      const pfxPath = path.join(absPath, file);
      const type = await fs.promises.stat(pfxPath);

      if (!type.isDirectory() || !isValidPrefix(pfxPath)) continue;

      const prefix = new Prefix(file, pfxPath);
      results.push(prefix);
    }

    return results;
  }
}

function isValidPrefix(_: string): boolean {
  // TODO
  return true;
}

export default Prefix;
