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

.PHONY: test
test: targets
	@mkdir -p $(TEST_REPORT_DIR)
	@rm -f $(TEST_RESULTS)
	@set +e; \
	run_one() { \
		language="$$1"; report_key="$$2"; command="$$3"; \
		log_file="$(TEST_REPORT_DIR)/$$report_key.log"; \
		printf '\n========================================\n %s Tests\n========================================\n' "$$language"; \
		start_time=$$(date +%s); \
		( eval "$$command" ) 2>&1 | tee "$$log_file"; \
		exit_status=$${PIPESTATUS[0]}; \
		duration=$$(( $$(date +%s) - start_time )); \
		case "$$report_key" in \
			rust|python) \
				passed=$$(grep -hoE '[0-9]+ passed' "$$log_file" | awk '{sum+=$$1} END{print sum+0}'); \
				failed=$$(grep -hoE '[0-9]+ failed' "$$log_file" | awk '{sum+=$$1} END{print sum+0}'); ;; \
			cs) \
				passed=$$(grep -hoE 'Passed:[[:space:]]*[0-9]+' "$$log_file" | grep -oE '[0-9]+$$' | awk '{sum+=$$1} END{print sum+0}'); \
				failed=$$(grep -hoE 'Failed:[[:space:]]*[0-9]+' "$$log_file" | grep -oE '[0-9]+$$' | awk '{sum+=$$1} END{print sum+0}'); ;; \
			go) \
				passed=$$(grep -cE '^[[:space:]]*--- PASS' "$$log_file"); \
				failed=$$(grep -cE '^[[:space:]]*--- FAIL' "$$log_file"); ;; \
			*) passed=0; failed=0; ;; \
		esac; \
		if grep -q '^SKIPPED:' "$$log_file"; then result=SKIP; passed='-'; failed='-'; \
		elif [ "$$exit_status" -eq 0 ]; then result=PASS; else result=FAIL; fi; \
		printf '%s|%s|%s|%s|%ss\n' "$$language" "$$result" "$${passed:-0}" "$${failed:-0}" "$$duration" >> $(TEST_RESULTS); \
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
	while IFS='|' read -r language result passed failed duration; do \
		printf '%-10s %-8s %-9s %-9s %-8s\n' "$$language" "$$result" "$$passed" "$$failed" "$$duration"; \
	done < $(TEST_RESULTS); \
	echo "==========================================================="; \
	if grep -q '|FAIL|' $(TEST_RESULTS); then \
		echo "OVERALL: FAIL"; exit 1; \
	else \
		echo "OVERALL: PASS"; \
	fi

.PHONY: test-rs
test-rs: check
	$(RUST_TEST_CMD)

.PHONY: test-cs
test-cs: ffi
	$(CS_TEST_CMD)

.PHONY: test-go
test-go: ffi
	$(GO_TEST_CMD)

.PHONY: test-python
test-python: ffi
	@$(PYTHON_TEST_CMD)

.PHONY: test-java
test-java: ffi
	@$(JAVA_TEST_CMD)

# ---------------------------------------------------------------------------
# Benchmarks. Results are appended to tests/benchmark_history/<lang>.csv keyed by the
# package version; set BENCH_RECORD=0 to skip recording, BENCH_ITERATIONS to change runs.
# ---------------------------------------------------------------------------
RS_VERSION      := $(shell sed -n 's/^version = "\(.*\)"/\1/p' rs/Cargo.toml | head -1)
BENCH_RECORD    ?= 1
BENCH_ITERATIONS ?= 5
BENCH_ENV       := BENCH_RECORD=$(BENCH_RECORD) BENCH_ITERATIONS=$(BENCH_ITERATIONS)
PARITY_BENCH_ITERATIONS ?= $(BENCH_ITERATIONS)
PARITY_BENCH_OUTPUT := $(abspath $(TARGET_DIR)/test-report/language-parity.csv)

.PHONY: bench-rs
bench-rs:
	$(BENCH_ENV) $(CARGO) bench -p draviavemal-openxml_office --bench footprint
	$(CARGO) bench -p draviavemal-openxml_office --bench spreadsheet -- --save-baseline v$(RS_VERSION)

# Compare the current code with a saved release baseline: make bench-rs-compare BENCH_BASELINE=v<version>
.PHONY: bench-rs-compare
bench-rs-compare:
	@test -n "$(BENCH_BASELINE)" || { echo "Set BENCH_BASELINE=v<version> (a name used with --save-baseline)"; exit 1; }
	$(CARGO) bench -p draviavemal-openxml_office --bench spreadsheet -- --baseline $(BENCH_BASELINE)

.PHONY: bench-go
bench-go: targets
	cd tests/go && $(BENCH_ENV) CGO_ENABLED=1 go test -run 'TestOpenEditSavePerformance|TestCompareWritePerformance' -v -count=1 ./...

.PHONY: bench-cs
bench-cs: targets
	cd tests/cs && $(BENCH_ENV) dotnet test $(DOTNET_CFG) --filter "FullyQualifiedName~Benchmark"

.PHONY: bench
bench: bench-rs bench-go bench-cs bench-parity

.PHONY: bench-parity
bench-parity:
	$(MAKE) BUILD_TYPE=release ffi
	@mkdir -p $(dir $(PARITY_BENCH_OUTPUT))
	@rm -f $(PARITY_BENCH_OUTPUT)
	PARITY_BENCH_OUTPUT=$(PARITY_BENCH_OUTPUT) PARITY_BENCH_ITERATIONS=$(PARITY_BENCH_ITERATIONS) $(CARGO) bench -p draviavemal-openxml_office --bench language_parity
	cd tests/go && PARITY_BENCH_OUTPUT=$(PARITY_BENCH_OUTPUT) PARITY_BENCH_ITERATIONS=$(PARITY_BENCH_ITERATIONS) CGO_ENABLED=1 go test -run '^TestLanguageParityBenchmark$$' -count=1 -v
	cd tests/cs && PARITY_BENCH_OUTPUT=$(PARITY_BENCH_OUTPUT) PARITY_BENCH_ITERATIONS=$(PARITY_BENCH_ITERATIONS) dotnet test --configuration Release --filter FullyQualifiedName~LanguageParityBenchmark
	@awk -F, 'NR > 1 { median_ms[$$1] = $$7 } END { if (median_ms["rust"] > 0) { printf "\nRelative throughput (Rust = 1.00x):\n"; printf "  Rust:   1.00x\n"; printf "  Go:     %.2fx\n", median_ms["rust"] / median_ms["go"]; printf "  C#:     %.2fx\n", median_ms["rust"] / median_ms["csharp"] } }' $(PARITY_BENCH_OUTPUT)
	@printf '\nCross-language timing results: %s\n' $(PARITY_BENCH_OUTPUT)
