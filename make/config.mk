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

WINDOWS_GNU_TARGET  := x86_64-pc-windows-gnu
WINDOWS_MSVC_TARGET := x86_64-pc-windows-msvc
LINUX_TARGET        := x86_64-unknown-linux-gnu
MACOS_TARGET        := x86_64-apple-darwin

FBS_DIR    := fbs
CS_DIR     := cs
JAVA_DIR   := java/draviavemal_openxml_office/src/main/java
PYTHON_DIR := python
GO_DIR     := go
TESTS_DIR  := tests

JAVA_RESOURCE_DIR := java/draviavemal_openxml_office/src/main/resources/lib

FFI_LIB_NAME := draviavemal_openxml_office_ffi
FFI_PACKAGE  := draviavemal-openxml_office_ffi

WINDOWS_BUILD_DIR := $(TARGET_DIR)/$(WINDOWS_GNU_TARGET)/$(CARGO_PROFILE)
LINUX_BUILD_DIR   := $(TARGET_DIR)/$(LINUX_TARGET)/$(CARGO_PROFILE)

WINDOWS_DLL     := $(WINDOWS_BUILD_DIR)/$(FFI_LIB_NAME).dll
WINDOWS_ARCHIVE := $(WINDOWS_BUILD_DIR)/lib$(FFI_LIB_NAME).a
LINUX_SO        := $(LINUX_BUILD_DIR)/lib$(FFI_LIB_NAME).so
LINUX_ARCHIVE   := $(LINUX_BUILD_DIR)/lib$(FFI_LIB_NAME).a

HOST_KERNEL := $(shell uname -s 2>/dev/null)
HOST_ARCH   := $(shell uname -m 2>/dev/null)

ifneq (,$(findstring Linux,$(HOST_KERNEL)))
	HOST_OS := linux
else ifneq (,$(findstring Darwin,$(HOST_KERNEL)))
	HOST_OS := macos
else ifneq (,$(findstring MINGW,$(HOST_KERNEL)))
	HOST_OS := windows
else ifneq (,$(findstring MSYS,$(HOST_KERNEL)))
	HOST_OS := windows
else ifneq (,$(findstring CYGWIN,$(HOST_KERNEL)))
	HOST_OS := windows
else ifneq (,$(OS))
	HOST_OS := windows
else
	HOST_OS := unknown
endif

ifeq ($(HOST_OS),linux)
	NATIVE_TARGET := $(LINUX_TARGET)
else ifeq ($(HOST_OS),windows)
	NATIVE_TARGET := $(WINDOWS_GNU_TARGET)
else ifeq ($(HOST_OS),macos)
	ifneq (,$(filter arm64 aarch64,$(HOST_ARCH)))
		NATIVE_TARGET := aarch64-apple-darwin
	else
		NATIVE_TARGET := $(MACOS_TARGET)
	endif
else
	NATIVE_TARGET :=
endif

NATIVE_DIST_DIR := $(TARGET_DIR)/native-dist

DEVOPS_BUILD ?=

FFI_PLATFORM_TARGETS := \
	$(if $(filter $(LINUX_TARGET),$(NATIVE_TARGET)),ffi-linux) \
	$(if $(filter $(WINDOWS_GNU_TARGET),$(NATIVE_TARGET)),ffi-windows)
