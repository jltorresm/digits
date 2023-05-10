.PHONY: clippy
clippy:
	cargo clippy --workspace -- -Wclippy::all -Wclippy::pedantic

.PHONY: run-cli
run-cli:
	cargo run --release --bin digits-cli

.PHONY: run-web
run-web:
	$(MAKE) -C web run-web
