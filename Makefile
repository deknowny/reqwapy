# Python executable path
python_exec = python3.8

# Poetry executable path
poetry_exec = $(python_exec) -m poetry

exec = .venv/bin/


build-debug:
	.venv/bin/maturin develop

check: build-debug
	.venv/bin/python .drafts/check.py


### Below is autogen, not tested
# Upgrade poetry, setup for installation
setup:
	$(python_exec) -m pip install --upgrade poetry

# Install project in production mode
install-production: setup
	$(poetry_exec) install --no-dev

# Install the project in a development mode
install-dev-all: setup
	$(poetry_exec) install -E dev-all && $(poetry_exec) run pre-commit install

# Install project with dev style dependencies
install-dev-style: setup
	$(poetry_exec) install -E dev-style && $(poetry_exec) run pre-commit install

# Install static checkers (i.e. mypy)
install-dev-check: setup
	$(poetry_exec) install -E dev-check

# Install docs builder dependencies
install-dev-docs: setup
	$(poetry_exec) install -E dev-docs

# Install tests tools
install-dev-test: setup
	$(poetry_exec) install -E dev-test

# Reformat code style
format:
	$(exec)black reqwapy && \
	git add -u && \
	$(exec)isort reqwapy && \
	git add -u && \
	$(exec)autoflake \
		--ignore-init-module-imports \
		--remove-unused-variables \
		--recursive \
		--in-place reqwapy tests && \
	git add -u

# Run tests locally
test:
	$(poetry_exec) run pytest tests --cov=reqwapy --cov-report=html

# Tests command for CI with .coveragerc report
test-ci:
	$(poetry_exec) run coverage run --source=reqwapy -m pytest tests

# Serve coverage report
serve-cov:
	python -m http.server -d htmlcov -b 127.0.0.1

# Run hot-reloaded docs server
serve-docs:
	$(poetry_exec) run mkdocs serve

# Deploy docs command
deploy-docs:
	$(poetry_exec) run mike deploy --push --update-aliases 0.1 latest -b gh-pages

# Run mypy checking
check-type:
	$(poetry_exec) run mypy
