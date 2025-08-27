
DEV_PORT = 5500
WASM_OUTDIR = $(CURDIR)/web/pkg/

.PHONY: server build build-release clean

serve:
	@if ! command -v wserver &> /dev/null; then cargo install wserver; fi
	wserver -l --path $(CURDIR)/web -p $(DEV_PORT)

build:
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --dev 
	@echo -e "\nDev build complete"

build-release:
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --release
	@echo -e "\nRelease build complete"

clean:
	rm -rf $(WASM_OUTDIR) ./target