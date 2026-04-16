# me — project commands

# Default: list available commands
default:
    @just --list

# ─── Development ─────────────────────────────────────────

# Start dev server on LAN and print the reachable URL
dev:
    @ip=$(ipconfig getifaddr en0 2>/dev/null || ipconfig getifaddr en1 2>/dev/null || echo localhost); \
    printf "\n  → http://%s:5173\n\n" "$ip"; \
    doppler run --project personal-website --config dev -- bun run dev --host --port 5173 --strictPort

# Type-check the project
check:
    bun run check

# Build for production
build:
    bun run build

# Preview the production build
preview:
    doppler run --project personal-website --config dev -- bun run preview

# Start the built server locally
start:
    doppler run --project personal-website --config dev -- node build

# Install dependencies
install:
    bun install

# Remove build artifacts and deps
clean:
    rm -rf build .svelte-kit node_modules

# ─── Database ────────────────────────────────────────────

# Create tables (idempotent)
migrate:
    doppler run --project personal-website --config dev -- bun run scripts/migrate.ts

# Import markdown journal entries from frontend/assets/journal
import-journal:
    doppler run --project personal-website --config dev -- bun run scripts/import-journal.ts

# Patch blog titles with acronym allowlist
fix-titles:
    doppler run --project personal-website --config dev -- bun run scripts/fix-titles.ts

# ─── Deploy ──────────────────────────────────────────────

# Sync Doppler secrets to Fly
sync-secrets:
    doppler secrets download --project personal-website --config dev --no-file --format env-no-quotes | fly secrets import

# Deploy to Fly.io
deploy:
    fly deploy
