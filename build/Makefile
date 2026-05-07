# hexa-cern — pandoc PDF rebuild for the 3 pillar specs.
#
# Generates the 3 pillar PDFs from the canonical .md sources via pandoc
# + xelatex. PDFs land in build/out/ and are .gitignore'd via the
# project root .gitignore.
#
# Usage:
#   make -C build check        # verify pandoc + xelatex available
#   make -C build all          # build all 3 pillar PDFs
#   make -C build mini         # mini pillar only
#   make -C build parent       # parent pillar only
#   make -C build classical    # classical pillar only
#   make -C build verify       # run `hexa-cern verify all` (cross-link)
#   make -C build clean        # remove build artifacts
#   make -C build size         # PDF size summary

ROOT     := ..
OUT      := out
PANDOC   := pandoc
HEADER   := header.tex
HEXA     := hexa

# CJK-aware xelatex pipeline. The header.tex include is soft-guarded —
# missing optional packages (xeCJK, titlesec) silently skip. Build-host
# without a CJK font still produces a clean ASCII-only PDF.
PANDOC_FLAGS := \
  --pdf-engine=xelatex \
  --variable=geometry:a4paper,margin=25mm \
  --variable=fontsize:11pt \
  --variable=colorlinks:true \
  --variable=linkcolor:black \
  --variable=urlcolor:blue \
  --syntax-highlighting=tango \
  --toc \
  --toc-depth=2

ifneq ($(wildcard $(HEADER)),)
  PANDOC_FLAGS += --include-in-header=$(HEADER)
endif

PDF_MINI      := $(OUT)/mini-accelerator.pdf
PDF_PARENT    := $(OUT)/particle-accelerator.pdf
PDF_CLASSICAL := $(OUT)/classical-mechanics-accelerator.pdf

PDFS := $(PDF_MINI) $(PDF_PARENT) $(PDF_CLASSICAL)

.PHONY: all clean check size mini parent classical verify

all: $(PDFS)

mini: $(PDF_MINI)
parent: $(PDF_PARENT)
classical: $(PDF_CLASSICAL)

$(OUT):
	mkdir -p $(OUT)

$(PDF_MINI): $(ROOT)/mini/doc/mini-accelerator.md | $(OUT)
	$(PANDOC) $(PANDOC_FLAGS) \
	  --metadata=title:"HEXA-MINI-ACCEL — benchtop laser-plasma 100 MeV / 1 GeV/m" \
	  --metadata=date:"2026-05-06" \
	  -o $@ $<

$(PDF_PARENT): $(ROOT)/parent/doc/particle-accelerator.md | $(OUT)
	$(PANDOC) $(PANDOC_FLAGS) \
	  --metadata=title:"HEXA-PACCEL — integrated parent particle accelerator" \
	  --metadata=date:"2026-05-06" \
	  -o $@ $<

$(PDF_CLASSICAL): $(ROOT)/classical/doc/classical-mechanics-accelerator.md | $(OUT)
	$(PANDOC) $(PANDOC_FLAGS) \
	  --metadata=title:"HEXA-CLASSIC-ACCEL — classical-mechanics baseline" \
	  --metadata=date:"2026-05-06" \
	  -o $@ $<

# Cross-link to the verify pipeline. `make verify` runs the full hexa-cern
# n=6 audit; useful pre-PDF as a regression gate.
verify:
	@echo "[build] running: hexa-cern verify all"
	@PETITE_CERN_ROOT=$(realpath $(ROOT)) $(HEXA) run $(ROOT)/cli/hexa-cern.hexa verify all

clean:
	rm -rf $(OUT)

size: $(PDFS)
	@du -h $(PDFS)

check:
	@command -v $(PANDOC)  >/dev/null 2>&1 || { echo "MISSING: pandoc";  exit 1; }
	@command -v xelatex    >/dev/null 2>&1 || { echo "MISSING: xelatex"; exit 1; }
	@command -v $(HEXA)    >/dev/null 2>&1 || { echo "MISSING: hexa (install via hx install)"; exit 1; }
	@$(PANDOC) --version | head -1
	@xelatex --version | head -1
	@$(HEXA)   version    | head -1
	@echo "OK: build toolchain ready"
