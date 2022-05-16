# Executables base path
EXEC_BASE = .venv/bin

# Base interprier, used for virtualenv
GLOBAL_PY = python3.8

# Local python executable from enviroment
LOCAL_PY = $(EXEC_BASE)/python

# Local pip executable from enviroment
LOCAL_PIP = $(EXEC_BASE)/pip

# Maturin package manager executable
MATURIN = $(EXEC_BASE)/python -m maturin

# Pytest executable
PYTEST = $(EXEC_BASE)/pytest

# The first command that always should be executed
# when you clone the source
setup:
	$(GLOBAL_PY) -m venv .venv && \
	$(LOCAL_PIP) install .


install-code-fix:
	$(LOCAL_PIP) install ".[code-fix]"

install-test:
	$(LOCAL_PIP) install ".[test]"

install-all:
	install-code-fix && install-test

build-debug:
	$(MATURIN) develop

build-prod:
	$(MATURIN) build

work-check:
	build-debug && $(LOCAL_PY) .drafts/check.py

format:
	$(EXEC_BASE)/black reqwapy && \
	git add -u && \
	$(EXEC_BASE)/isort reqwapy && \
	git add -u && \
	$(EXEC_BASE)/autoflake \
		--ignore-init-module-imports \
		--remove-unused-variables \
		--recursive \
		--in-place reqwapy tests && \
	git add -u

# Run tests locally
test:
	$(PYTEST) tests --cov=reqwapy --cov-report=html

# Tests command for CI with .coveragerc report
test-ci:
	$(EXEC_BASE)/coverage run --source=reqwapy -m pytest tests

# Serve coverage report
serve-cov:
	$(LOCAL_PY) -m http.server -d htmlcov -b 127.0.0.1

# Run hot-reloaded docs server
serve-docs:
	$(EXEC_BASE)/mkdocs serve

# Deploy docs command
deploy-docs:
	$(poetry_exec) run mike deploy --push --update-aliases 0.1 latest -b gh-pages

# Run mypy checking
type-check:
	$(EXEC_BASE)/mypy run
