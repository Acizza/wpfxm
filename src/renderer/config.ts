import React, { useState } from "react";

// Optional fields must explicitly have an undefined variant, otherwise it will not be loaded properly
export interface Config {
  prefixPath: string | undefined;
}

export function defaultConfig(): Config {
  return {
    prefixPath: undefined,
  };
}

export interface ConfigState {
  config: Config;
  setValue: <T extends keyof Config>(
    key: T,
    value: Config[keyof Config]
  ) => void;
}

export const ConfigContext = React.createContext<ConfigState | undefined>(
  undefined
);

export function useGlobalConfig(): ConfigState {
  const [config, setConfig] = useState(loadConfig());

  function setValue<T extends keyof Config>(
    key: T,
    value: Config[keyof Config]
  ) {
    setConfig((cfg) => {
      cfg[key] = value;
      localStorage.setItem(key, value as any);
      return cfg;
    });
  }

  return {
    config,
    setValue,
  };
}

export function loadConfig(): Config {
  let config = defaultConfig();

  Object.keys(config).forEach((key) => {
    const item = localStorage.getItem(key);
    if (item) (config as any)[key] = item;
  });

  return config;
}

loadConfig();
