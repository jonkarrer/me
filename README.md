# Me

An experimental portfolio

## Development

Use these steps to get going:

- Clone the repository
- Install dependencies, `cargo build`

### Scripts

All the scripts are in the `justfile`

To start the server with hot reloading, run `just start-server`.

## Infastructure

### Backend

This is the file router system. It will handle reaching out to data stores and passing data to the frontend. `Askama` is used to render templates, which live in `frontend/templates`, pointed to by the `askama.toml` file.

### Frontend

This is where the html and css files are stored. This is where website content changes are made.

### Cli

This is a tool to help manage the database.
