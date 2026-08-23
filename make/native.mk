.PHONY: rust-core
rust-core: check
	$(CARGO) build $(CARGO_FLAG) \
		--target $(NATIVE_TARGET)

.PHONY: native
native: check
	DEVOPS_BUILD=$(DEVOPS_BUILD) $(CARGO) build $(CARGO_FLAG) \
		-p $(FFI_PACKAGE) \
		--target $(NATIVE_TARGET)

	rm -rf $(NATIVE_DIST_DIR)/$(NATIVE_TARGET)
	mkdir -p $(NATIVE_DIST_DIR)/$(NATIVE_TARGET) $(NATIVE_DIST_DIR)/include

	@build_dir="$(TARGET_DIR)/$(NATIVE_TARGET)/$(CARGO_PROFILE)"; \
	dest_dir="$(NATIVE_DIST_DIR)/$(NATIVE_TARGET)"; \
	for lib_file in \
		"lib$(FFI_LIB_NAME).so" "lib$(FFI_LIB_NAME).dylib" "$(FFI_LIB_NAME).dll" \
		"lib$(FFI_LIB_NAME).a" "$(FFI_LIB_NAME).lib" "lib$(FFI_LIB_NAME).lib"; do \
		[ -f "$$build_dir/$$lib_file" ] && cp "$$build_dir/$$lib_file" "$$dest_dir/" && echo "  + $$lib_file"; \
	done; \
	true

	-cp $(TARGET_DIR)/methods.h $(NATIVE_DIST_DIR)/include/ 2>/dev/null || true
	-cp $(TARGET_DIR)/headers.h $(NATIVE_DIST_DIR)/include/ 2>/dev/null || true

	@find $(NATIVE_DIST_DIR) -type f | sort

.PHONY: ffi
ffi: flatc rust-core ffi-prep $(FFI_PLATFORM_TARGETS)
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
	$(CARGO) build $(CARGO_FLAG) \
		--target $(LINUX_TARGET)

	cp $(LINUX_SO) \
		$(CS_DIR)/runtimes/linux-x64/native/lib$(FFI_LIB_NAME).so

	cp $(LINUX_SO) \
		$(PYTHON_DIR)/lib/lib$(FFI_LIB_NAME).so

	cp $(LINUX_SO) \
		$(JAVA_RESOURCE_DIR)/lib$(FFI_LIB_NAME).so

	cp $(LINUX_ARCHIVE) \
		$(GO_DIR)/lib/lib$(FFI_LIB_NAME).a

.PHONY: ffi-windows
ffi-windows: check ffi-prep
	$(CARGO) build $(CARGO_FLAG) \
		--target $(WINDOWS_GNU_TARGET)

	cp $(WINDOWS_DLL) \
		$(CS_DIR)/runtimes/win-x64/native/$(FFI_LIB_NAME).dll

	cp $(WINDOWS_DLL) \
		$(PYTHON_DIR)/lib/$(FFI_LIB_NAME).dll

	cp $(WINDOWS_DLL) \
		$(JAVA_RESOURCE_DIR)/$(FFI_LIB_NAME).dll

	cp $(WINDOWS_ARCHIVE) \
		$(GO_DIR)/lib/lib$(FFI_LIB_NAME)-windows-amd64.a \
		2>/dev/null || true

.PHONY: windows-msvc
windows-msvc: check
	$(CARGO) build $(CARGO_FLAG) \
		--target $(WINDOWS_MSVC_TARGET)
