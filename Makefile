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
MAC_TARGET   := x86_64-apple-darwin
MSVC_TARGET  := x86_64-pc-windows-msvc

FBS_DIR    := fbs
CS_DIR     := cs
JAVA_DIR   := java/draviavemal_openxml_office/src/main/java
PYTHON_DIR := python
GO_DIR     := go
TESTS      := tests

JAVA_RESOURCE_DIR := java/draviavemal_openxml_office/src/main/resources/lib

FFI_NAME := draviavemal_openxml_office_ffi

WIN_BUILD_DIR   := $(TARGET_DIR)/$(WIN_TARGET)/$(CARGO_PROFILE)
LINUX_BUILD_DIR := $(TARGET_DIR)/$(LINUX_TARGET)/$(CARGO_PROFILE)

WIN_DLL  := $(WIN_BUILD_DIR)/$(FFI_NAME).dll
WIN_A    := $(WIN_BUILD_DIR)/lib$(FFI_NAME).a
LINUX_SO := $(LINUX_BUILD_DIR)/lib$(FFI_NAME).so
LINUX_A  := $(LINUX_BUILD_DIR)/lib$(FFI_NAME).a


# ============================================================
# Host detection
# ============================================================

UNAME_S := $(shell uname -s 2>/dev/null)
UNAME_M := $(shell uname -m 2>/dev/null)

ifneq (,$(findstring Linux,$(UNAME_S)))
	HOST_OS := linux
else ifneq (,$(findstring Darwin,$(UNAME_S)))
	HOST_OS := macos
else ifneq (,$(findstring MINGW,$(UNAME_S)))
	HOST_OS := windows
else ifneq (,$(findstring MSYS,$(UNAME_S)))
	HOST_OS := windows
else ifneq (,$(findstring CYGWIN,$(UNAME_S)))
	HOST_OS := windows
else ifneq (,$(OS))
	HOST_OS := windows
else
	HOST_OS := unknown
endif

# Resolve the native Rust target triple for this host.
ifeq ($(HOST_OS),linux)
	NATIVE_TARGET := $(LINUX_TARGET)
else ifeq ($(HOST_OS),windows)
	NATIVE_TARGET := $(WIN_TARGET)
else ifeq ($(HOST_OS),macos)
	ifneq (,$(filter arm64 aarch64,$(UNAME_M)))
		NATIVE_TARGET := aarch64-apple-darwin
	else
		NATIVE_TARGET := $(MAC_TARGET)
	endif
else
	NATIVE_TARGET :=
endif


# ============================================================
# Build target
# ------------------------------------------------------------
# Both debug and release compile for the host OS only, so the
# native target is the only target.
# ============================================================

FFI_PLATFORM_TARGETS := \
	$(if $(filter $(LINUX_TARGET),$(NATIVE_TARGET)),ffi-linux) \
	$(if $(filter $(WIN_TARGET),$(NATIVE_TARGET)),ffi-windows)


# ============================================================
# Default: SHOW HELP
# ============================================================

.PHONY: help
help:
	@echo "  make              Show build tree"
	@echo "Build targets:"
	@echo "  make all          Full DEBUG build (host OS only)"
	@echo "  make release      Full RELEASE build (host OS only)"
	@echo "Stages:"
	@echo "  make check  		Check tool versions"
	@echo "  make flatc        Generate FlatBuffer sources"
	@echo "  make rust-core    Build Rust workspace (host target)"
	@echo "  make ffi          Build FFI (flatbuffer + rust-core)"
	@echo "  make cs           Build C# (ffi)"
	@echo "  make python       Build Python (ffi)"
	@echo "  make java         Build Java (ffi)"
	@echo "  make go           Build Go (ffi)"
	@echo "  make test         Run tests for all languages (summary report)"
	@echo "  make clean        Remove build artifacts"
	@echo ""
	@echo "Detected host      : $(HOST_OS) ($(UNAME_M))"
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
# Tool version report (flatc required, rest optional)
# ============================================================

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
	@echo "Host: $(HOST_OS) ($(UNAME_M)) | Build: $(BUILD_TYPE) | Target: $(NATIVE_TARGET)"


# ============================================================
# Clean
# ============================================================

.PHONY: clean
clean:
	@echo "Cleaning generated/build artifacts..."

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

	rm -rf $(TESTS)/cs/bin
	rm -rf $(TESTS)/cs/obj
	rm -rf $(TESTS)/cs/edit_test_files
	rm -rf $(TESTS)/cs/test_results
	rm -rf $(TESTS)/go/test_results

# ============================================================
# FlatBuffer
# ============================================================

.PHONY: flatc
flatc: check
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
rust-core: check
	@echo ""
	@echo "========================================"
	@echo " Rust Workspace"
	@echo " Target: $(NATIVE_TARGET)"
	@echo " Profile: $(CARGO_PROFILE)"
	@echo "========================================"

	$(CARGO) build $(CARGO_FLAG) \
		--target $(NATIVE_TARGET)


# ============================================================
# FFI
# ------------------------------------------------------------
# Builds the FFI artifact for the host target and deploys it
# to each language binding.
# ============================================================

.PHONY: ffi
ffi: flatc rust-core ffi-prep $(FFI_PLATFORM_TARGETS)
	@echo ""
	@echo "========================================"
	@echo " Rust FFI ($(NATIVE_TARGET))"
	@echo "========================================"

	cp $(TARGET_DIR)/headers.h \
		$(GO_DIR)/lib/headers.h

	cp $(TARGET_DIR)/methods.h \
		$(PYTHON_DIR)/lib/methods.h

.PHONY: ffi-prep
ffi-prep:
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

.PHONY: ffi-linux
ffi-linux: check ffi-prep
	@echo "-> Building Linux FFI ($(LINUX_TARGET))"
	$(CARGO) build $(CARGO_FLAG) \
		--target $(LINUX_TARGET)

	cp $(LINUX_SO) \
		$(CS_DIR)/runtimes/linux-x64/native/lib$(FFI_NAME).so

	cp $(LINUX_SO) \
		$(PYTHON_DIR)/lib/lib$(FFI_NAME).so

	cp $(LINUX_SO) \
		$(JAVA_RESOURCE_DIR)/lib$(FFI_NAME).so

	cp $(LINUX_A) \
		$(GO_DIR)/lib/lib$(FFI_NAME).a

.PHONY: ffi-windows
ffi-windows: check ffi-prep
	@echo "-> Building Windows FFI ($(WIN_TARGET))"
	$(CARGO) build $(CARGO_FLAG) \
		--target $(WIN_TARGET)

	cp $(WIN_DLL) \
		$(CS_DIR)/runtimes/win-x64/native/$(FFI_NAME).dll

	cp $(WIN_DLL) \
		$(PYTHON_DIR)/lib/$(FFI_NAME).dll

	cp $(WIN_DLL) \
		$(JAVA_RESOURCE_DIR)/$(FFI_NAME).dll

	cp $(WIN_A) \
		$(GO_DIR)/lib/lib$(FFI_NAME)-windows-amd64.a \
		2>/dev/null || true


# ============================================================
# Optional Windows MSVC
# ============================================================

.PHONY: windows-msvc
windows-msvc: check
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
	@echo " Go Build"
	@echo "========================================"

	cd $(GO_DIR) && \
	CGO_ENABLED=1 go build ./...


# ============================================================
# All target languages
# ============================================================

.PHONY: targets
targets: cs python java go


# ============================================================
# Tests (run after build, not part of build prep)
# ============================================================

TEST_REPORT_DIR := $(TARGET_DIR)/test-report
TEST_RESULTS    := $(TEST_REPORT_DIR)/results.txt

ifeq ($(BUILD_TYPE),release)
	DOTNET_CFG := --configuration Release
else
	DOTNET_CFG :=
endif

RUST_TEST_CMD := $(CARGO) test $(CARGO_FLAG) --target $(NATIVE_TARGET)
CS_TEST_CMD   := cd tests/cs && dotnet test $(DOTNET_CFG)
GO_TEST_CMD   := cd tests/go && CGO_ENABLED=1 go test ./... -v -count=1

ifneq ($(wildcard tests/python/*),)
	PYTHON_TEST_CMD := cd $(PYTHON_DIR) && python3 -m pytest ../tests/python
else
	PYTHON_TEST_CMD := echo 'SKIPPED: no Python tests found'
endif

ifneq ($(wildcard java/draviavemal_openxml_office/gradlew),)
	JAVA_TEST_CMD := cd java/draviavemal_openxml_office && ./gradlew test
else
	JAVA_TEST_CMD := echo 'SKIPPED: gradle wrapper not found'
endif

# Run every language test suite, then print a consolidated summary.
.PHONY: test
test: targets
	@mkdir -p $(TEST_REPORT_DIR)
	@rm -f $(TEST_RESULTS)
	@set +e; \
	run_one() { \
		label="$$1"; key="$$2"; cmd="$$3"; \
		log="$(TEST_REPORT_DIR)/$$key.log"; \
		printf '\n========================================\n %s Tests\n========================================\n' "$$label"; \
		start=$$(date +%s); \
		( eval "$$cmd" ) 2>&1 | tee "$$log"; \
		st=$${PIPESTATUS[0]}; \
		dur=$$(( $$(date +%s) - start )); \
		case "$$key" in \
			rust|python) \
				p=$$(grep -hoE '[0-9]+ passed' "$$log" | awk '{s+=$$1} END{print s+0}'); \
				f=$$(grep -hoE '[0-9]+ failed' "$$log" | awk '{s+=$$1} END{print s+0}'); ;; \
			cs) \
				p=$$(grep -hoE 'Passed:[[:space:]]*[0-9]+' "$$log" | grep -oE '[0-9]+$$' | awk '{s+=$$1} END{print s+0}'); \
				f=$$(grep -hoE 'Failed:[[:space:]]*[0-9]+' "$$log" | grep -oE '[0-9]+$$' | awk '{s+=$$1} END{print s+0}'); ;; \
			go) \
				p=$$(grep -cE '^[[:space:]]*--- PASS' "$$log"); \
				f=$$(grep -cE '^[[:space:]]*--- FAIL' "$$log"); ;; \
			*) p=0; f=0; ;; \
		esac; \
		if grep -q '^SKIPPED:' "$$log"; then res=SKIP; p='-'; f='-'; \
		elif [ "$$st" -eq 0 ]; then res=PASS; else res=FAIL; fi; \
		printf '%s|%s|%s|%s|%ss\n' "$$label" "$$res" "$${p:-0}" "$${f:-0}" "$$dur" >> $(TEST_RESULTS); \
	}; \
	run_one "Rust"   rust   "$(RUST_TEST_CMD)"; \
	run_one "C#"     cs     "$(CS_TEST_CMD)"; \
	run_one "Go"     go     "$(GO_TEST_CMD)"; \
	run_one "Python" python "$(PYTHON_TEST_CMD)"; \
	run_one "Java"   java   "$(JAVA_TEST_CMD)"; \
	echo ""; \
	echo "======================= TEST SUMMARY ======================="; \
	printf '%-10s %-8s %-9s %-9s %-8s\n' "Language" "Result" "Passed" "Failed" "Time"; \
	printf '%-10s %-8s %-9s %-9s %-8s\n' "----------" "--------" "---------" "---------" "--------"; \
	while IFS='|' read -r l r p f t; do \
		printf '%-10s %-8s %-9s %-9s %-8s\n' "$$l" "$$r" "$$p" "$$f" "$$t"; \
	done < $(TEST_RESULTS); \
	echo "==========================================================="; \
	if grep -q '|FAIL|' $(TEST_RESULTS); then \
		echo "OVERALL: FAIL"; exit 1; \
	else \
		echo "OVERALL: PASS"; \
	fi

.PHONY: test-rs
test-rs: check
	@printf '\n========================================\n Rust Tests\n========================================\n'
	$(RUST_TEST_CMD)

.PHONY: test-cs
test-cs: ffi
	@printf '\n========================================\n C# Tests\n========================================\n'
	$(CS_TEST_CMD)

.PHONY: test-go
test-go: ffi
	@printf '\n========================================\n Go Tests\n========================================\n'
	$(GO_TEST_CMD)

.PHONY: test-python
test-python: ffi
	@printf '\n========================================\n Python Tests\n========================================\n'
	@$(PYTHON_TEST_CMD)

.PHONY: test-java
test-java: ffi
	@printf '\n========================================\n Java Tests\n========================================\n'
	@$(JAVA_TEST_CMD)