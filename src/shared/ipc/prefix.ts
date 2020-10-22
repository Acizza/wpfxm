export interface Prefix {
  name: string;
  path: string;
  arch: PrefixArch;
}

export const enum PrefixArch {
  X32,
  X64,
}
