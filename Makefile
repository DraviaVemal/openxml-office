SHELL := /bin/bash

# ============================================================
# Configuration
# ============================================================

BUILD_TYPE ?= debug

ifeq ($(BUILD_TYPE),release)
	CARGO_FLAG := --release
	CARGO_PROFILE := release
else
	CARGO_FLAG :=
	CARGO_PROFILE := debug
endif

CARGO := cargo

TARGET_DIR := target

WIN_TARGET   := x86_64-pc-windows-gnu
LINUX_TARGET := x86_64-unknown-linux-gnu
MSVC_TARGET  := x86_64-pc-windows-msvc

FBS_DIR    := fbs
CS_DIR     := cs
JAVA_DIR   := java/draviavemal_openxml_office/src/main/java
PYTHON_DIR := python
GO_DIR     := go

JAVA_RESOURCE_DIR := java/draviavemal_openxml_office/src/main/resources/lib

FFI_NAME := draviavemal_openxml_office_ffi

WIN_BUILD_DIR   := $(TARGET_DIR)/$(WIN_TARGET)/$(CARGO_PROFILE)
LINUX_BUILD_DIR := $(TARGET_DIR)/$(LINUX_TARGET)/$(CARGO_PROFILE)

WIN_DLL  := $(WIN_BUILD_DIR)/$(FFI_NAME).dll
WIN_A    := $(WIN_BUILD_DIR)/lib$(FFI_NAME).a
LINUX_SO := $(LINUX_BUILD_DIR)/lib$(FFI_NAME).so
LINUX_A  := $(LINUX_BUILD_DIR)/lib$(FFI_NAME).a


# ============================================================
# Default: SHOW HELP
# ============================================================

.PHONY: help
help:
	@echo "Build targets:"
	@echo "  make              Show build tree"
	@echo "  make all          Full DEBUG build"
	@echo "  make release      Full RELEASE build"
	@echo "Stages:"
	@echo "  make flatc   Generate FlatBuffer sources"
	@echo "  make rust-core    Build Rust workspace"
	@echo "  make ffi          Build FFI (flatbuffer + rust-core)"
	@echo "  make cs           Build C# (ffi)"
	@echo "  make python       Build Python (ffi)"
	@echo "  make java         Build Java (ffi)"
	@echo "  make go           Build/test Go (ffi)"
	@echo "  make targets      Build all target languages"
	@echo "  make clean        Remove build artifacts"
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


# ============================================================
# Full build
# ============================================================

.PHONY: all
all: targets


# ============================================================
# Release shortcut
# ============================================================

.PHONY: release
release:
	$(MAKE) BUILD_TYPE=release all


# ============================================================
# Clean
# ============================================================

.PHONY: clean
clean:
	@echo "Cleaning generated/build artifacts..."

	rm -rf $(TARGET_DIR)

	rm -rf $(CS_DIR)/openxml_office_fbs
	rm -rf $(GO_DIR)/internal/openxml_office_fbs
	rm -rf $(JAVA_DIR)/openxml_office_fbs
	rm -rf $(PYTHON_DIR)/openxml_office_fbs

	rm -rf $(CS_DIR)/runtimes
	rm -rf $(PYTHON_DIR)/lib
	rm -rf java/draviavemal_openxml_office/src/lib
	rm -rf $(GO_DIR)/lib


# ============================================================
# FlatBuffer
# ============================================================

.PHONY: flatc
flatc:
	@echo ""
	@echo "========================================"
	@echo " FlatBuffer Generation"
	@echo "========================================"

	rm -rf $(CS_DIR)/openxml_office_fbs
	rm -rf $(GO_DIR)/internal/openxml_office_fbs
	rm -rf $(JAVA_DIR)/openxml_office_fbs
	rm -rf $(PYTHON_DIR)/openxml_office_fbs

	mkdir -p $(CS_DIR)
	mkdir -p $(GO_DIR)/internal
	mkdir -p $(JAVA_DIR)/openxml_office_fbs
	mkdir -p $(PYTHON_DIR)/openxml_office_fbs

	find $(FBS_DIR) -name "*.fbs" -print0 | \
	while IFS= read -r -d '' fbs_file; do \
		echo "Generating $$fbs_file"; \
		flatc -n -o $(CS_DIR) "$$fbs_file"; \
		flatc -j -o $(JAVA_DIR) "$$fbs_file"; \
	done

	flatc -g --gen-all \
		--go-module-name draviavemal_openxml_office/internal \
		-o $(GO_DIR)/internal \
		$(FBS_DIR)/consolidated.fbs

	flatc -p --gen-all \
		-o $(PYTHON_DIR) \
		$(FBS_DIR)/consolidated.fbs


# ============================================================
# Rust workspace
# ============================================================

.PHONY: rust-core
rust-core:
	@echo ""
	@echo "========================================"
	@echo " Rust Workspace"
	@echo " Target: $(LINUX_TARGET)"
	@echo " Profile: $(CARGO_PROFILE)"
	@echo "========================================"

	$(CARGO) build $(CARGO_FLAG) \
		--target $(LINUX_TARGET)


# ============================================================
# FFI
# ============================================================

.PHONY: ffi
ffi: flatc rust-core
	@echo ""
	@echo "========================================"
	@echo " Rust FFI"
	@echo "========================================"

	rm -rf $(CS_DIR)/runtimes
	mkdir -p \
		$(CS_DIR)/runtimes/win-x64/native \
		$(CS_DIR)/runtimes/linux-x64/native

	rm -rf $(PYTHON_DIR)/lib
	mkdir -p $(PYTHON_DIR)/lib

	rm -rf $(JAVA_RESOURCE_DIR)
	mkdir -p $(JAVA_RESOURCE_DIR)

	rm -rf $(GO_DIR)/lib
	mkdir -p $(GO_DIR)/lib

	$(CARGO) build $(CARGO_FLAG) \
		--target $(WIN_TARGET)

	$(CARGO) build $(CARGO_FLAG) \
		--target $(LINUX_TARGET)

	cp $(WIN_DLL) \
		$(CS_DIR)/runtimes/win-x64/native/$(FFI_NAME).dll

	cp $(WIN_DLL) \
		$(PYTHON_DIR)/lib/$(FFI_NAME).dll

	cp $(WIN_DLL) \
		$(JAVA_RESOURCE_DIR)/$(FFI_NAME).dll

	cp $(LINUX_SO) \
		$(CS_DIR)/runtimes/linux-x64/native/lib$(FFI_NAME).so

	cp $(LINUX_SO) \
		$(PYTHON_DIR)/lib/lib$(FFI_NAME).so

	cp $(LINUX_SO) \
		$(JAVA_RESOURCE_DIR)/lib$(FFI_NAME).so

	cp $(LINUX_A) \
		$(GO_DIR)/lib/lib$(FFI_NAME).a

	cp $(WIN_A) \
		$(GO_DIR)/lib/lib$(FFI_NAME)-windows-amd64.a \
		2>/dev/null || true

	cp $(TARGET_DIR)/headers.h \
		$(GO_DIR)/lib/headers.h

	cp $(TARGET_DIR)/methods.h \
		$(PYTHON_DIR)/lib/methods.h


# ============================================================
# Optional Windows MSVC
# ============================================================

.PHONY: windows-msvc
windows-msvc:
	$(CARGO) build $(CARGO_FLAG) \
		--target $(MSVC_TARGET)


# ============================================================
# C#
# ============================================================

.PHONY: cs
cs: ffi
	@echo ""
	@echo "========================================"
	@echo " C# Build"
	@echo "========================================"

	cd $(CS_DIR) && \
	if [ "$(BUILD_TYPE)" = "release" ]; then \
		dotnet build --configuration Release; \
	else \
		dotnet build; \
	fi


# ============================================================
# Python
# ============================================================

.PHONY: python
python: ffi
	@echo ""
	@echo "========================================"
	@echo " Python Build"
	@echo "========================================"

	cd $(PYTHON_DIR) && \
	python3 setup.py bdist_wheel


# ============================================================
# Java
# ============================================================

.PHONY: java
java: ffi
	@echo ""
	@echo "========================================"
	@echo " Java Build"
	@echo "========================================"

	@if [ -f java/draviavemal_openxml_office/gradlew ]; then \
		cd java/draviavemal_openxml_office && ./gradlew build; \
	else \
		echo "Java Gradle wrapper not found."; \
	fi


# ============================================================
# Go
# ============================================================

.PHONY: go
go: ffi
	@echo ""
	@echo "========================================"
	@echo " Go Test"
	@echo "========================================"

	cd tests/go && \
	CGO_ENABLED=1 go test ./...


# ============================================================
# All target languages
# ============================================================

.PHONY: targets
targets: cs python java go