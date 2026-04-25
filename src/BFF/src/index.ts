import { Hono } from 'hono';

const app = new Hono();

app.get('/health', (c) => c.json({ status: 'ok', service: 'bff' }));

const port = process.env.PORT || 4000;
console.log(`🚀 BFF running on http://localhost:${port}`);

export default {
  port,
  fetch: app.fetch,
};
