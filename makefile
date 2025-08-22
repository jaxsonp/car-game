
WASM_OUTDIR = $(PWD)/web/pkg/

.PHONY: build build-release clean

build:
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --dev 

build-release:
	wasm-pack build ./car-game/ --target web --out-dir $(WASM_OUTDIR) --release

clean:
	rm -rf $(WASM_OUTDIR) ./target