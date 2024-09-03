# Me

An experimental portfolio

## Development

Use these steps to get going:

- Clone the repository
- Install dependencies, `cargo build`
- Install tailwindcss binary at `frontend/tailwindcss`

### Scripts

All the scripts are in the `justfile`

To start a server that has Hot Reloading and a style engine watcher, run `just start-server` in one terminal and `just start-tailwind` in another.

## Infastructure

### Backend

This is the file router system. It will handle reaching out to data stores and passing data to the frontend. `Askama` is used to render templates, which live in `frontend/templates`, pointed to by the `askama.toml` file.

### Frontend

This is where the html and css files are stored, as well as the tailwind binary and config. This is where website content changes are made.

### Cli

This is a tool to help manage the database. 