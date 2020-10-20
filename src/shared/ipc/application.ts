import { IPrefix } from "./prefix";

export const maxAppEvents = 500;

export interface FoundApplications {
  paths: ApplicationPath[];
  commonPathPrefix: string;
}

export interface ApplicationPath {
  absolute: string;
  stripped: string;
}

export interface Application {
  prefix: IPrefix;
  path: ApplicationPath;
}

export interface LaunchOptions {
  app: Application;
  args?: string[];
  env?: { [key: string]: string };
  force32Bit: boolean;
}

type OutputEvent = { kind: "out"; data: string };
type LaunchEvent = { kind: "launch"; app: Application };
type ClosedEvent = { kind: "close"; app: Application };

export type AppEvent = OutputEvent | LaunchEvent | ClosedEvent;

export default {};
