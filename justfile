start-server:
    cargo watch -qcx 'shuttle run'
start-tailwind:
    frontend/tailwindcss -c frontend/tailwind.config.js -i frontend/styles/input.css -o frontend/styles/output.css --watch

# Database CLI
init-journal-table:
    cargo run --bin cli -- init

refresh-journal-table:
    cargo run --bin cli -- refresh

list-journal-table:
    cargo run --bin cli -- list

add-journal-table title summary:
    cargo run --bin cli -- add "{{title}}" "{{summary}}"
