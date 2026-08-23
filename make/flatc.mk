.PHONY: flatc
flatc:
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

.PHONY: bindings
bindings: flatc
	mkdir -p $(NATIVE_DIST_DIR)
	tar -czf $(NATIVE_DIST_DIR)/bindings.tar.gz \
		$(CS_DIR)/openxml_office_fbs \
		$(JAVA_DIR)/openxml_office_fbs \
		$(GO_DIR)/internal \
		$(PYTHON_DIR)/openxml_office_fbs
