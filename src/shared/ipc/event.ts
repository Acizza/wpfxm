export const enum IPC {
  ScanPrefixes = "scan-prefixes",
  ScanPrefixApps = "scan-prefix-apps",
  LaunchApp = "launch-app",
  CloseApp = "close-app",
  AppEvent = "app-event",
}

export const enum IPCSync {
  NormalizePath = "normalize-path",
  FileExists = "file-exists",
  GetAppEvents = "get-app-events",
}
