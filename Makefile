clippy:
	cargo clippy -- -Wclippy::all -Wclippy::pedantic

run-cli:
	cargo run --release --bin digits-cli
