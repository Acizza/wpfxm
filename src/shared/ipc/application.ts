import { IPrefix } from "./prefix";

export interface FoundApplications {
  paths: ApplicationPath[];
  commonPathPrefix: string;
}

export interface ApplicationPath {
  absolute: string;
  stripped: string;
}

export interface SelectedApp {
  prefix: IPrefix;
  path: ApplicationPath;
}

export interface LaunchOptions {
  prefix: IPrefix;
  path: string;
  args?: string[];
  env?: { [key: string]: string };
  force32Bit: boolean;
}

type DataEvent = { kind: "data"; data: string };
type ClosedEvent = { kind: "closed"; success: boolean };

export type EventKind = DataEvent | ClosedEvent;

export default {};
