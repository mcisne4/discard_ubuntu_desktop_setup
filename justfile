# === DEV === #
dev: build-shell && build-sqlite-db build-dev
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

# === MODULE TREE === #
mods: mods-tauri && mods-dev mods-shell mods-sqlite-db

mods-dev:
  cargo-modules generate tree -p rs_dev --types --traits --fns

mods-tauri:
  cargo-modules generate tree -p rs_tauri --types --traits --fns

mods-shell:
  cargo-modules generate tree -p rs_shell --types --traits --fns

mods-sqlite-db:
  cargo-modules generate tree -p rs_sqlite_db --types --traits --fns