start-postgres:
    sudo systemctl start postgresql

write-schema:
    diesel print-schema > src/db/schema.rs

watch:
    watchexec -c --exts rs,hbs --restart "cargo run --bin locutions-server"
