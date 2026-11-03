# me — jonkarrer.com project commands
#
# Secrets live in a local .env (gitignored; copy .env.example to start).
# `set dotenv-load` puts them in the environment of every recipe, so
# `bun run …` and `fly secrets import` see them without any wrapper.

set dotenv-load

# Default: list available commands
default:
    @just --list

# ─── Development ─────────────────────────────────────────

# Start dev server on LAN and print the reachable URL
dev:
    @ip=$(ipconfig getifaddr en0 2>/dev/null || ipconfig getifaddr en1 2>/dev/null || echo localhost); \
    printf "\n  → http://%s:5173\n\n" "$ip"; \
    bun run dev --host --port 5173 --strictPort

# Type-check the project
check:
    bun run check

# Build for production
build:
    bun run build

# Preview the production build
preview:
    bun run preview

# Start the built server locally
start:
    bun build/index.js

# Install dependencies
install:
    bun install

# Remove build artifacts and deps
clean:
    rm -rf build .svelte-kit node_modules

# ─── Database ────────────────────────────────────────────

# Create tables (idempotent)
migrate:
    bun run scripts/migrate.ts

# Import every markdown journal entry from frontend/assets/journal
import-journal:
    bun run scripts/import-journal.ts

# Publish or update one journal entry: just add-entry <slug> "<title>" "<description>"
add-entry slug title description:
    bun run scripts/add-entry.ts "{{slug}}" "{{title}}" "{{description}}"

# Patch blog titles with acronym allowlist
fix-titles:
    bun run scripts/fix-titles.ts

# ─── Deploy ──────────────────────────────────────────────

# Push .env to the Fly app's secret store (app: jonkarrer, org: personal)
sync-secrets:
    grep -E '^[A-Z_]+=.+' .env | fly secrets import -a jonkarrer

# Deploy to Fly.io
deploy:
    fly deploy

# Tail production logs
logs:
    fly logs -a jonkarrer

# Open the live site
open:
    open https://jonkarrer.com
