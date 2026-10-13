import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";

const backendDir = fileURLToPath(new URL("../backend", import.meta.url));

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, backendDir, "SERVER_");
  const port = env.SERVER_PORT ?? "3000";

  return {
    plugins: [react()],
    build: {
      outDir: "../backend/static",
      emptyOutDir: true,
    },
    server: {
      proxy: {
        "/api": {
          target: `http://localhost:${port}`,
          rewrite: (path) => path.replace(/^\/api/, ""),
        },
      },
    },
  };
});
