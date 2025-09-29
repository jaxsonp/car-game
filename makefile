
DEV_PORT = 5500
WASM_OUTDIR = $(CURDIR)/web/pkg/

.PHONY: wasm-pack build build-release serve clean

wasm-pack:
	@if ! which wasm-pack &> /dev/null; then echo "installing wasm-pack" && cargo install wasm-pack; fi

build: wasm-pack
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --dev 
	tsc --project $(CURDIR)
	@echo -e "\nDev build complete"

build-release: wasm-pack
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --release
	tsc --project $(CURDIR)
	@echo -e "\nRelease build complete"

serve:
	@if ! which wserver &> /dev/null; then cargo install wserver; fi
	wserver -l --path $(CURDIR)/web -p $(DEV_PORT)

clean:
	rm -rf $(WASM_OUTDIR) $(CURDIR)/web/scripts/*.js $(CURDIR)/target
