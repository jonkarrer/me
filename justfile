start-server:
    shuttle run
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

remove-journal-table id:
    cargo run --bin cli -- remove "{{id}}"

update-summary-journal-table id summary:
    cargo run --bin cli -- update-summary "{{id}}" "{{summary}}"

update-title-journal-table id title:
    cargo run --bin cli -- update-title "{{id}}" "{{title}}"

update-content-journal-table id file_name:
    cargo run --bin cli -- update-content "{{id}}" "{{file_name}}"

