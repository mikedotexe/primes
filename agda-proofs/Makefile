# Makefile for Local Agda Verification
# Mirrors the CI workflow for local development

AGDA := agda
AGDA_FLAGS := --no-main --library=standard-library
AGDA_HTML_FLAGS := --html

# Core abstract framework modules (Layer 1)
CORE_MODULES := \
	Theorems/Abstract/SymmetryImpliesRepulsion.agda \
	Theorems/Abstract/SymmetryFromList.agda \
	Theorems/TotientDensity.agda

# Modules that need additional fixes
# PENDING_MODULES := \
# 	Theorems/Abstract/ConstrainedOrbitals.agda \
# 	Theorems/Abstract/BucketsAutoMatch.agda \
# 	Theorems/Abstract/WindowCertificate.agda

.PHONY: all core clean html help check-agda

all: check-agda core

# Check that Agda and stdlib are properly installed
check-agda:
	@echo "🔍 Checking Agda installation..."
	@command -v $(AGDA) >/dev/null 2>&1 || { echo "❌ Agda not found. Please install Agda 2.8.0+"; exit 1; }
	@$(AGDA) --version
	@echo "✓ Agda found"
	@echo ""
	@echo "📚 Checking standard library setup..."
	@test -f ~/.agda/libraries || { echo "❌ Agda libraries file not found"; exit 1; }
	@grep -q standard-library ~/.agda/libraries || { echo "❌ Standard library not registered"; exit 1; }
	@echo "✓ Standard library registered"

# Verify core abstract framework
core: check-agda
	@echo "🔬 Verifying core abstract framework (Layer 1)..."
	@for module in $(CORE_MODULES); do \
		echo "  Checking $$module..."; \
		$(AGDA) $(AGDA_FLAGS) $$module || exit 1; \
	done
	@echo "✅ Core framework verified successfully!"

# Generate HTML documentation
html: core
	@echo "📚 Generating HTML documentation..."
	@for module in $(CORE_MODULES); do \
		echo "  Generating HTML for $$module..."; \
		$(AGDA) $(AGDA_HTML_FLAGS) $$module || exit 1; \
	done
	@echo "✅ HTML documentation generated in html/"

# Clean generated files
clean:
	@echo "🧹 Cleaning generated files..."
	@find . -name "*.agdai" -delete
	@rm -rf html/
	@echo "✓ Clean complete"

# Help target
help:
	@echo "Agda Verification Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  make all        - Check setup and verify core modules (default)"
	@echo "  make core       - Verify core abstract framework"
	@echo "  make html       - Generate HTML documentation"
	@echo "  make clean      - Remove generated files (.agdai, html/)"
	@echo "  make check-agda - Verify Agda and stdlib installation"
	@echo "  make help       - Show this help message"
	@echo ""
	@echo "Setup instructions:"
	@echo "  1. Install Agda 2.8.0+"
	@echo "  2. Install standard-library v2.3+"
	@echo "  3. Register project library:"
	@echo "     echo \"\$$(pwd)/prime-physics-engine.agda-lib\" >> ~/.agda/libraries"
	@echo ""
	@echo "Quick verification:"
	@echo "  make          # Verify all working modules"
	@echo "  make html     # Generate browsable proof documentation"
