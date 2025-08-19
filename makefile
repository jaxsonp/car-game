
WASM_OUTDIR = ./web/pkg/

.PHONY: build build-release clean

build:
	wasm-pack build --target web --out-dir $(WASM_OUTDIR) --dev 

build-release:
	wasm-pack build --target web --out-dir $(WASM_OUTDIR) --release

clean:
	rm -rf $(WASM_OUTDIR) ./target