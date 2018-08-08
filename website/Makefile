NPM := `npm bin`
PWD := `pwd`
DIST := $(PWD)/dist

install: node_modules
	git submodule update --init --recursive
	curl https://sh.rustup.rs -sSf | sh
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	rustup component add rustfmt-preview --toolchain nightly
	cargo install --force cargo-watch
	cargo install --force cargo-web
.PHONY: install

build: cargo webpack css
.PHONY: build

clean: clean-cargo clean-webpack clean-css
.PHONY: clean

start:
	$(NPM)/concurrently \
		--raw \
		--kill-others \
		"$(MAKE) watch-cargo" \
		"$(MAKE) watch-webpack" \
		"$(MAKE) watch-css" \
		"$(MAKE) start-server"
.PHONY: start

test:
	echo TODO
.PHONY: test

node_modules:
	yarn install

cargo:
	cargo web deploy \
		--target=wasm32-unknown-unknown \
		--release \
		--verbose
.PHONY: cargo

clean-cargo:
	rm -Rf target/deploy
.PHONY: clean-cargo

full-clean-cargo:
	rm -Rf target
.PHONY: full-clean-cargo

watch-cargo:
	cargo watch \
		-i '*.css' \
		-i '*.html' \
		-i '*.js*' \
		-i '*.sh' \
		-i 'node_modules' \
		-x "web deploy --target=wasm32-unknown-unknown --release --verbose"
.PHONY: watch-cargo

$(DIST):
	mkdir -p $(DIST)

webpack: $(DIST)
	NODE_ENV=production $(NPM)/webpack
.PHONY: webpack

clean-webpack:
	rm -Rf $(DIST)/index.js
	rm -Rf $(DIST)/index.wasm
	rm -Rf $(DIST)/index.html
.PHONY: clean-webpack

watch-webpack: $(DIST)
	$(NPM)/webpack --watch
.PHONY: watch-webpack

css: $(DIST)
	$(NPM)/postcss \
        static/index.css \
        --output $(DIST)/index.css
.PHONY: css

clean-css:
	rm -Rf $(DIST)/index.css
.PHONY: clean-css

watch-css: $(DIST)
	$(NPM)/postcss \
        static/index.css \
        --output $(DIST)/index.css \
		--watch
.PHONY: watch-css

start-server:
	$(NPM)/browser-sync start \
		--port 1337 \
		--https \
		--single \
		--server $(DIST) \
		--no-open \
		--no-ui \
		--watch
