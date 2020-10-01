import * as path from "path";
import * as os from "os";

export function normalizeUnixPath(value: string): string {
  if (!value.length) return value;

  const slice =
    value[0] === "~" ? path.join(os.homedir(), value.slice(1)) : value;

  return path.normalize(slice);
}
