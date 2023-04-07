all:

grep-todo:
	@grep -n -e "TODO:" -e "FIXME:" -r --exclude="Makefile" --exclude-dir=".venv" . || true

rust-version:
	@rustc --version

rust-update:
	@rustup update

rust-sample-build:
	@rustc main.rs

rust-sample-exec:
	@rustc main

rust-cargo-version:
	@cargo --version

rust-cargo-init:
	@cargo init

rust-cargo-add-linter:
	@rustup component add rustfmt
	@rustup component add clippy
	@rustup component add rls  # language server. vscode拡張でrust analyzer もインストールする

rust-format:
	@cargo fmt
	@cargo fix --allow-dirty
	@cargo clippy --fix --allow-dirty

rust-test:
	@cargo test
