// @ts-ignore

import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";
import {resolve} from 'path'

// ts-expect-error process is a nodejs global
// const host = process.env.TAURI_DEV_HOST;
const serverHost = "0.0.0.0";
const devHost = "192.168.137.1";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    build:{
        target: ['es2022', 'chrome89', 'firefox89', 'safari15', 'edge89']
    },
    resolve: {
        alias: {
            '@': resolve(__dirname, "./src"),
            '@cp': resolve(__dirname, "./src/components"),
            '@view': resolve(__dirname, "./src/views"),
            '@lib': resolve(__dirname, "./src/libs"),
        }
    },

    plugins: [vue()],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: serverHost || false,
        hmr: devHost
            ? {
                protocol: "ws",
                host: devHost,
                port: 1421,
            }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
}));
