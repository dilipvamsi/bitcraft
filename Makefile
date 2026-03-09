# Add cargo bin to path
export PATH := $(HOME)/.cargo/bin:$(PATH)

.PHONY: all test normal fuzz perf clean help expand expand-sample

# Default target
all: normal fuzz

## --- TEST SUITES ---

# Run all "normal" tests (unit tests, doc-tests, and core integration tests)
normal:
	@echo "==> Running Normal Tests (Unit + Doc + Core Integration)"
	cargo test --lib
	cargo test --doc
	cargo test --test byte_tests
	cargo test --test example_control_register_test
	cargo test --test example_protocol_header_test
	cargo test --test debug_verify

# Run property-based fuzz tests
fuzz:
	@echo "==> Running Property-Based Fuzz Tests"
	cargo test --test fuzz_properties

# Run hardware-aligned performance benchmarks (Release mode required)
perf:
	@echo "==> Running Performance Benchmarks (Release Mode)"
	cargo test --release --test performance -- --nocapture

## --- UTILITIES ---

# Run clippy for static analysis
lint:
	@echo "==> Running Clippy"
	cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	@echo "==> Cleaning Build Artifacts"
	cargo clean

# Show help
help:
	@echo "bitstruct - Zero-Cost Bitfield Engine"
	@echo ""
	@echo "Usage:"
	@echo "  make normal      Run unit tests, doc-tests, and core integration tests."
	@echo "  make fuzz        Run property-based fuzz tests (proptest)."
	@echo "  make perf        Run performance benchmarks in release mode."
	@echo "  make test        Alias for 'make normal'."
	@echo "  make lint        Run clippy with strict warnings."
	@echo "  make all         Run both normal and fuzz tests."
	@echo "  make clean       Remove all build artifacts."

# Show macro expansion
expand:
	@echo "==> Expanding VehicleState"
	cargo expand tests::VehicleState

# Convert sample/sample.rs to sample/expanded_sample.rs
expand-sample:
	@echo "==> Expanding bitstruct engine for inspection (Selective)"
	cargo expand --example sample 2>/dev/null | python3 sample/filter_expand.py > sample/expanded_sample.rs || true
	@echo "Generated sample/expanded_sample.rs with selective expansion"
