# Executables base path
EXEC_BASE = .venv/bin

# Base interprier, used for virtualenv
GLOBAL_PY = python3.8

# Local python executable from enviroment
LOCAL_PY = $(EXEC_BASE)/python

# Local pip executable from enviroment
LOCAL_PIP = $(EXEC_BASE)/pip

# Maturin package manager executable
MATURIN = $(EXEC_BASE)/maturin

# Pytest executable
PYTEST = $(EXEC_BASE)/pytest

# The first command that always should be executed
# when you clone the source
setup:
	rm -rf .venv && \
	$(GLOBAL_PY) -m pip install --upgrade pip && \
	$(GLOBAL_PY) -m venv .venv && \
	$(LOCAL_PIP) install --upgrade pip . && \

enable-enviroment:
	. $(EXEC_BASE)/activate

install-code-fix:
	$(LOCAL_PIP) install ".[code-fix]"

install-test:
	$(LOCAL_PIP) install ".[test]"

install-build:
	$(LOCAL_PIP) install ".[build]"

install-all:
	$(MAKE) install-code-fix && $(MAKE) install-test && $(MAKE) install-build

build-debug:
	$(MAKE) enable-enviroment && $(MATURIN) develop $(ARGS)

build-prod:
	$(MAKE) enable-enviroment && $(MATURIN) build

work-check: build-debug
	$(LOCAL_PY) .drafts/check.py

format:
	$(LOCAL_PY) -m black reqwapy && \
	git add -u && \
	$(LOCAL_PY) -m isort reqwapy && \
	git add -u && \
	$(LOCAL_PY) -m autoflake \
		--ignore-init-module-imports \
		--remove-unused-variables \
		--recursive \
		--in-place reqwapy tests && \
	git add -u

benchmark:
	$(PYTEST) benchmarks $(ARGS) --benchmark-max-time=10

# Run tests locally
test:
	$(PYTEST) tests --cov=reqwapy --cov-report=html

# Tests command for CI with .coveragerc report
test-ci:
	$(LOCAL_PY) -m coverage run --source=reqwapy -m pytest tests

# Serve coverage report
serve-cov:
	$(LOCAL_PY) -m http.server -d htmlcov -b 127.0.0.1

# Run hot-reloaded docs server
serve-docs:
	$(LOCAL_PY) -m mkdocs serve

# Deploy docs command
deploy-docs:
	$(LOCAL_PY) -m mike deploy --push --update-aliases 0.1 latest -b gh-pages

# Run mypy checking
type-check:
	$(EXEC_BASE)/mypy run
