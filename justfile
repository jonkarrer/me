start-server:
    doppler run -- cargo run --bin me
start-tailwind:
    frontend/tailwindcss -c frontend/tailwind.config.js -i frontend/styles/input.css -o frontend/styles/output.css --watch

# Database CLI
init-journal-table:
    cargo run --bin cli -- init

refresh-journal-table:
    cargo run --bin cli -- refresh

list-journal-table:
    cargo run --bin cli -- list

add-journal-entry title summary:
    cargo run --bin cli -- add "{{title}}" "{{summary}}"

remove-journal-table id:
    cargo run --bin cli -- remove "{{id}}"

update-summary-journal-table id summary:
    cargo run --bin cli -- update-summary "{{id}}" "{{summary}}"

update-title-journal-table id title:
    cargo run --bin cli -- update-title "{{id}}" "{{title}}"

update-content-journal-table id file_name:
    cargo run --bin cli -- update-content "{{id}}" "{{file_name}}"

# Deploy
deploy:
  git pull && \
  sudo docker build -t me . && \
  sudo docker run --env-file .env -p 5105:5105 -d --restart unless-stopped me