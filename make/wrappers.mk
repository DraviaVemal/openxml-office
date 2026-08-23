.PHONY: cs
cs: ffi
	cd $(CS_DIR) && \
	if [ "$(BUILD_TYPE)" = "release" ]; then \
		dotnet build --configuration Release; \
	else \
		dotnet build; \
	fi

.PHONY: python
python: ffi
	cd $(PYTHON_DIR) && \
	python3 setup.py bdist_wheel

.PHONY: java
java: ffi
	@if [ -f java/draviavemal_openxml_office/gradlew ]; then \
		cd java/draviavemal_openxml_office && ./gradlew build; \
	else \
		echo "Java Gradle wrapper not found."; \
	fi

.PHONY: go
go: ffi
	cd $(GO_DIR) && \
	CGO_ENABLED=1 go build ./...

.PHONY: targets
targets: cs python java go
