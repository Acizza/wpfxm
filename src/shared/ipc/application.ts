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

export interface SelectedApp {
  prefix: IPrefix;
  path: ApplicationPath;
}

export interface LaunchOptions {
  app: SelectedApp;
  args?: string[];
  env?: { [key: string]: string };
  force32Bit: boolean;
}

type OutputEvent = { kind: "out"; data: string };
type LaunchEvent = { kind: "launch"; prefix: IPrefix };
type ClosedEvent = { kind: "close"; prefix: IPrefix };

export type AppEvent = OutputEvent | LaunchEvent | ClosedEvent;

export default {};
