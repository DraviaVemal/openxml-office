SHELL := /bin/bash

.DEFAULT_GOAL := help

include make/config.mk
include make/flatc.mk
include make/native.mk
include make/wrappers.mk
include make/test.mk

.PHONY: help
help:
	@echo "  make              Show build tree"
	@echo "Build targets:"
	@echo "  make all          Full DEBUG build (host OS only)"
	@echo "  make release      Full RELEASE build (host OS only)"
	@echo "Stages:"
	@echo "  make check        Check tool versions"
	@echo "  make flatc        Generate FlatBuffer sources"
	@echo "  make bindings     Generate + bundle bindings (CI share artifact)"
	@echo "  make rust-core    Build Rust workspace (host target)"
	@echo "  make native       Build host FFI lib into target/native-dist (CI)"
	@echo "  make ffi          Build FFI + deploy into every binding (local dev)"
	@echo "  make cs           Build C# (ffi)"
	@echo "  make python       Build Python (ffi)"
	@echo "  make java         Build Java (ffi)"
	@echo "  make go           Build Go (ffi)"
	@echo "  make test         Run tests for all languages (summary report)"
	@echo "  make clean        Remove build artifacts"
	@echo ""
	@echo "Detected host      : $(HOST_OS) ($(HOST_ARCH))"
	@echo "Native target      : $(NATIVE_TARGET)"
	@echo "Active build type  : $(BUILD_TYPE)"
	@echo ""
	@echo "Build tree:"
	@echo "  all"
	@echo "  ├── rust-core"
	@echo "  ├── flatc"
	@echo "  │   └── ffi"
	@echo "  └── targets"
	@echo "      ├── cs"
	@echo "      ├── python"
	@echo "      ├── java"
	@echo "      └── go"
	@echo "Dependencies are built automatically."

.PHONY: all
all: targets

.PHONY: release
release:
	$(MAKE) BUILD_TYPE=release all

.PHONY: check
check:
	@command -v flatc >/dev/null 2>&1 || { \
		echo "ERROR: flatc (FlatBuffers compiler) not found. Install from https://github.com/google/flatbuffers"; exit 1; }
	@echo "Installed tool versions:"
	@printf "  flatc  : "; flatc --version
	@printf "  rustc  : "; command -v rustc   >/dev/null 2>&1 && rustc --version           || echo "not installed"
	@printf "  cargo  : "; command -v cargo   >/dev/null 2>&1 && cargo --version           || echo "not installed"
	@printf "  go     : "; command -v go      >/dev/null 2>&1 && go version                || echo "not installed"
	@printf "  python : "; command -v python3 >/dev/null 2>&1 && python3 --version         || echo "not installed"
	@printf "  java   : "; command -v java    >/dev/null 2>&1 && java -version 2>&1 | head -1 || echo "not installed"
	@printf "  dotnet : "; command -v dotnet  >/dev/null 2>&1 && dotnet --version          || echo "not installed"
	@echo "Host: $(HOST_OS) ($(HOST_ARCH)) | Build: $(BUILD_TYPE) | Target: $(NATIVE_TARGET)"

.PHONY: clean
clean:
	rm -rf $(TARGET_DIR)

	rm -rf $(CS_DIR)/openxml_office_fbs
	rm -rf $(CS_DIR)/runtimes
	rm -rf $(CS_DIR)/bin
	rm -rf $(CS_DIR)/obj

	rm -rf $(PYTHON_DIR)/openxml_office_fbs
	rm -rf $(PYTHON_DIR)/lib
	rm -rf $(PYTHON_DIR)/.eggs
	rm -rf $(PYTHON_DIR)/build
	rm -rf $(PYTHON_DIR)/dist
	rm -rf $(PYTHON_DIR)/draviavemal_openxml_office.egg-info

	rm -rf $(JAVA_DIR)/openxml_office_fbs
	rm -rf java/draviavemal_openxml_office/src/lib

	rm -rf $(GO_DIR)/internal
	rm -rf $(GO_DIR)/lib

	rm -rf $(TESTS_DIR)/cs/bin
	rm -rf $(TESTS_DIR)/cs/obj
	rm -rf $(TESTS_DIR)/cs/edit_test_files
	rm -rf $(TESTS_DIR)/cs/test_results
	rm -rf $(TESTS_DIR)/go/test_results
