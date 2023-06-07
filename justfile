# === DEV === #
dev: build-logger && build-log-details build-shell build-sqlite-db build-dev
  cd rs_dev && cargo watch -x run

# === BUILD SCRIPTS === #
build:
  cargo build

build-dev:
  cargo build -p rs_dev

build-tauri:
  cargo build -p rs_tauri

build-shell:
  cargo build -p rs_shell

build-sqlite-db:
  cargo build -p rs_sqlite_db

build-logger:
  cargo build -p rs_logger

build-log-details:
  cargo build -p rs_log_details

# === MODULE TREE === #
mods: mods-tauri && mods-dev mods-shell mods-sqlite-db mods-logs mods-log-details

mods-dev:
  cargo-modules generate tree -p rs_dev --types --traits --fns

mods-tauri:
  cargo-modules generate tree -p rs_tauri --types --traits --fns

mods-shell:
  cargo-modules generate tree -p rs_shell --types --traits --fns

mods-sqlite-db:
  cargo-modules generate tree -p rs_sqlite_db --types --traits --fns

mods-logs:
  cargo-modules generate tree -p rs_logger --types --traits --fns

mods-log-details:
  cargo-modules generate tree -p rs_log_details --types --traits --fns
