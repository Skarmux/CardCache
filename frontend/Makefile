build:
	@RUSTFLAGS= trunk build --release

clean:
	@cargo clean

web:
	@RUSTFLAGS= trunk serve --release --watch tailwind.css

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	touch src/**
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: build web test docs style-check lint
