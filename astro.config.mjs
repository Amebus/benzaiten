// @ts-check
import { defineConfig } from 'astro/config';

import react from '@astrojs/react';

import vercel from '@astrojs/vercel';
import bun from "@nurodev/astro-bun";

// https://astro.build/config
export default defineConfig({
  output: 'server',
  integrations: [react({
    experimentalReactChildren: true,
  })],
  adapter: bun(),
  // adapter: vercel(),
  devToolbar: {
    enabled: true,
  }
});