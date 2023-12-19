import { join, resolve } from "path";
import { defineConfig } from "vite";

export default defineConfig({
  base: "/advent-of-code-2023/",
  build: {
    rollupOptions: {
      input: ["", "day01"].map((dir) =>
        resolve(__dirname, join(dir, "index.html")),
      ),
    },
  },
});
