dev: build
  cd rs_dev && cargo watch -x run

build:
  cargo build

mods-dev:
  cargo-modules generate tree -p rs_dev --types --traits --fns

mods-tauri:
  cargo-modules generate tree -p rs_tauri --types --traits --fns

mods-shell:
  cargo-modules generate tree -p rs_shell --types --traits --fns

mods: mods-tauri && mods-dev mods-shell
