export const enum IPC {
  ScanPrefixes = "scan-prefixes",
  ScanPrefixApps = "scan-prefix-apps",
  LaunchProcess = "launch-process",
  AppEvent = "app-event",
}

export const enum IPCSync {
  NormalizePath = "normalize-path",
  FileExists = "file-exists",
  GetAppEvents = "get-app-events",
}
