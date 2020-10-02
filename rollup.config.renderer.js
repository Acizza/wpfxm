import replace from "@rollup/plugin-replace";
import resolve from "@rollup/plugin-node-resolve";
import postcss from "rollup-plugin-postcss";
import babel from "@rollup/plugin-babel";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import { terser } from "rollup-plugin-terser";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import { config } from "./package.json";

const production = !process.env.ROLLUP_WATCH;
const extensions = [".js", ".jsx", ".ts", ".tsx"];

export default {
  input: "src/renderer/index.tsx",
  output: {
    file: "public/build/renderer_main.js",
    format: "cjs",
    sourcemap: true,
  },
  plugins: [
    replace({
      "process.env.NODE_ENV": JSON.stringify(
        production ? "production" : "development"
      ),
    }),
    resolve({
      extensions,
    }),
    postcss({
      extract: false,
      modules: true,
      use: ["sass"],
      minimize: production,
    }),
    babel({
      babelHelpers: "bundled",
      extensions,
      include: "src/**/*",
    }),
    commonjs(),
    json(),
    production && terser(),
    !production &&
      serve({
        contentBase: "public",
        port: config.dev_port,
      }),
    !production && livereload("public"),
  ],
};
