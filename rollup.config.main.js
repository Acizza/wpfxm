import replace from "@rollup/plugin-replace";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import babel from "@rollup/plugin-babel";
import { terser } from "rollup-plugin-terser";
import builtins from "builtin-modules";
import json from "@rollup/plugin-json";

const production = !process.env.ROLLUP_WATCH;
const extensions = [".js", ".ts"];

const plugins = [
  replace({
    "process.env.NODE_ENV": JSON.stringify(
      production ? "production" : "development"
    ),
  }),
  resolve({ extensions }),
  babel({
    babelHelpers: "bundled",
    extensions,
    include: ["src/main/**/*", "src/shared/**/*"],
  }),
  commonjs(),
  json(),
  production && terser(),
];

const external = [...builtins, "electron"];

export default [
  {
    input: "src/main/main.ts",
    output: {
      file: "public/build/electron_main.js",
      format: "cjs",
      sourcemap: true,
    },
    plugins,
    external,
  },
  {
    input: "src/main/preload.ts",
    output: {
      file: "public/build/electron_preload.js",
      format: "cjs",
      sourcemap: true,
    },
    plugins,
    external,
  },
];
