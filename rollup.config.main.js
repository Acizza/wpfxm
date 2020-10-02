import replace from "@rollup/plugin-replace";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import babel from "@rollup/plugin-babel";
import { terser } from "rollup-plugin-terser";
import builtins from "builtin-modules";

const production = !process.env.ROLLUP_WATCH;

export default {
  input: "src/main/main.ts",
  output: {
    file: "public/build/electron_main.js",
    format: "cjs",
    sourcemap: true,
  },
  plugins: [
    replace({
      "process.env.NODE_ENV": JSON.stringify(
        production ? "production" : "development"
      ),
    }),
    babel({
      babelHelpers: "bundled",
      extensions: [".js", ".jsx", ".ts", ".tsx"],
      include: ["src/main/**/*"],
    }),
    resolve(),
    commonjs(),
    production && terser(),
  ],
  external: [builtins, "electron"],
};
