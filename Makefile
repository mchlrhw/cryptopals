build:
	cargo build

test:
	cargo fmt -- --check
	cargo clippy
	cargo test
	poetry run pytest --black --cov=cryptopals --flake8 --isort --mypy

fmt:
	cargo fmt
	poetry run isort -y
	poetry run black $(PWD)

.PHONY: test fmt
