# Me

An experimental portfolio.

## Development

- Clone the repository
- `just install` to install dependencies
- `just dev` to start the dev server on your LAN

All commands live in the `justfile` — run `just` to list them.

Secrets come from Doppler (`personal-website` / `dev`), so the recipes wrap
commands in `doppler run`.

## Infrastructure

### App

SvelteKit (Svelte 5) served by `adapter-node`, running on Bun. Routes are in
`src/routes`: the home page, the journal list, and the `[slug]` entry page.
Styling is a single hand-rolled stylesheet at `src/app.css`.

### Database

Turso (libSQL). `src/lib/server/db.ts` holds the client, the `blogs` schema, and
the queries; `src/hooks.server.ts` ensures the tables exist at boot.

### Content

Journal entries are markdown files in `frontend/assets/journal`, which is the
source of truth. Publish one with:

    just import-journal            # bulk import every file
    bun run scripts/add-entry.ts <slug> "<title>" "<description>"

The routes then read the rendered entries out of the database.

### Deploy

Fly.io (app `jonkarrer`). Pushing to `main` triggers the GitHub Actions deploy;
`just deploy` does it by hand, and `just sync-secrets` pushes Doppler secrets to
Fly.
