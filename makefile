
DEV_PORT = 5500

.PHONY: wasm-pack build build-release serve clean

wasm-pack:
	@if ! which wasm-pack &> /dev/null; then echo "installing wasm-pack" && cargo install wasm-pack; fi

build: wasm-pack
	wasm-pack build ./car-game/ --target web --out-dir $(CURDIR)/web/pkg --dev 
	tsc --project $(CURDIR)
	@echo -e "\nDev build complete"

build-release: wasm-pack
	wasm-pack build ./car-game/ --target web --out-dir $(CURDIR)/dist/pkg/ --release
	tsc --project $(CURDIR)
	cp -r $(CURDIR)/web/. $(CURDIR)/dist
	rm $(CURDIR)/dist/**/*.ts
	@echo -e "\nRelease build complete"

serve:
	@if ! which wserver &> /dev/null; then cargo install wserver; fi
	wserver -l --path $(CURDIR)/web -p $(DEV_PORT)

clean:
	rm -rf $(WASM_OUTDIR) $(CURDIR)/web/scripts/*.js $(CURDIR)/target $(CURDIR)/dist
