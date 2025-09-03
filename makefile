
DEV_PORT = 5500
WASM_OUTDIR = $(CURDIR)/web/pkg/

.PHONY: install-wasm-pack build build-release server clean

install-wasm-pack:
	@if ! which wasm-pack &> /dev/null; then echo "installing wasm-pack" && cargo install wasm-pack; fi

build: install-wasm-pack
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --dev 
	@echo -e "\nDev build complete"

build-release: install-wasm-pack
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --release
	@echo -e "\nRelease build complete"

serve:
	@if ! which wserver &> /dev/null; then cargo install wserver; fi
	wserver -l --path $(CURDIR)/web -p $(DEV_PORT)

clean:
	rm -rf $(WASM_OUTDIR) ./target