test:
	poetry run pytest --black --cov=cryptopals --flake8 --isort --mypy

fmt:
	poetry run isort -y
	poetry run black $(PWD)

.PHONY: test fmt
