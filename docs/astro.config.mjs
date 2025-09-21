// astro.config.mjs
import { defineConfig } from 'astro/config';
import tailwind from "@tailwindcss/vite";

export default defineConfig({
    base: '/fib/',
    integrations: [tailwind()],
    output: 'static'
});
