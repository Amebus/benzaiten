# Benzaiten

This project now runs on Next.js with the App Router, so pages render server-side by default and client interactivity is isolated to React components that opt into `use client`.

## Commands

Run all commands from the project root:

| Command | Action |
| :------ | :----- |
| `bun install` | Install dependencies |
| `bun run dev` | Start the Next.js development server |
| `bun run build` | Build the production app |
| `bun run start` | Start the production server |
| `bun run lint` | Run ESLint |

## Structure

- `src/app`: Next.js routes and root layout.
- `src/components`: reusable React components.
- `src/basic`: generic utility and base React components.
- `public`: static assets served as-is.
