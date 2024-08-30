start:
    cargo watch -qcx 'shuttle run'

# Database CLI
init-journal-table:
    cargo run --bin database -- init

refresh-journal-table:
    cargo run --bin database -- refresh

list-journal-table:
    cargo run --bin database -- list

add-journal-table title summary:
    cargo run --bin database -- add "{{title}}" "{{summary}}"
