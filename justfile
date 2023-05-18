dev: build-shell && build-dev
  cd rs_dev && cargo watch -x run

build:
  cargo build

build-dev:
  cargo build -p rs_dev

build-tauri:
  cargo build -p rs_tauri

build-shell:
  cargo build -p rs_shell

mods-dev:
  cargo-modules generate tree -p rs_dev --types --traits --fns

mods-tauri:
  cargo-modules generate tree -p rs_tauri --types --traits --fns

mods-shell:
  cargo-modules generate tree -p rs_shell --types --traits --fns

mods: mods-tauri && mods-dev mods-shell
