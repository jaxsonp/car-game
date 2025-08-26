
DEV_PORT = 5500
WASM_OUTDIR = $(CURDIR)/web/pkg/

.PHONY: run-dev build-dev build-release clean

run: build-dev
	@if ! command -v wserver &> /dev/null; then cargo install wserver; fi
	wserver --path $(CURDIR)/web -p $(DEV_PORT)

build-dev:
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --dev 

build-release:
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --release

clean:
	rm -rf $(WASM_OUTDIR) ./target