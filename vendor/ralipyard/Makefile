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
	$(LOCAL_PIP) install --upgrade pip maturin

enable-environment:
	. $(EXEC_BASE)/activate

install-code-fix:
	$(LOCAL_PIP) install ".[code-fix]"

install-test:
	$(LOCAL_PIP) install ".[test]"

install-build:
	$(LOCAL_PIP) install ".[build]"

install-all:
	$(MAKE) install-code-fix && $(MAKE) install-test && $(MAKE) install-build

develop-debug: enable-environment
	$(MATURIN) develop $(ARGS)

develop-release: enable-environment
	$(MATURIN) build

work-check: develop-debug
	$(LOCAL_PY) .drafts/check.py

test:
	$(PYTEST)