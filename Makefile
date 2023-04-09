all:

grep-todo:
	@grep -n -e "TODO:" -e "FIXME:" -r --exclude="Makefile" --exclude-dir=".venv" . || true

rust-version:
	@rustc --version

rust-update:
	@rustup update

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

run-echo-server:
	@cargo run

run-client:
	@nc 127.0.0.1 8080

# build-debug:
# 	@rm -f pysrc/*.so
# 	@cargo build
# 	@rm -f target/debug/lib_rust_sample.d
# 	@rm -f target/debug/lib_rust_sample.rlib
# 	@mv target/debug/lib_rust_sample.* pysrc/_rust_sample.so
