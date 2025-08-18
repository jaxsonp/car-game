
WASM_OUTDIR = ./web/pkg

.PHONY: wasm wasm-release clean

wasm:
	wasm-pack build --target web --out-dir $(WASM_OUTDIR) --dev

wasm-release:
	wasm-pack build --target web --out-dir $(WASM_OUTDIR) --release

clean:
	rm -rf $(WASM_OUTDIR) ./target