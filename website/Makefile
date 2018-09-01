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
	cargo install --force wasm-gc
	cargo install --force wasm-snip
.PHONY: install

build: clean css cargo webpack
.PHONY: build

clean:
	rm -Rf dist
	mkdir -p dist
.PHONY: clean

start: build
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
	cargo web build \
		--target=wasm32-unknown-unknown \
		--release \
		--verbose
	wasm-gc target/wasm32-unknown-unknown/release/eosstrawpoll.wasm target/wasm32-unknown-unknown/release/eosstrawpoll.wasm
	wasm-opt --output target/wasm32-unknown-unknown/release/eosstrawpoll.wasm -Oz target/wasm32-unknown-unknown/release/eosstrawpoll.wasm
.PHONY: cargo

clean-cargo:
	rm -Rf target/wasm32-unknown-unknown
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
		-x "web build --target=wasm32-unknown-unknown --release --verbose"
.PHONY: watch-cargo

$(DIST):
	mkdir -p $(DIST)

webpack: $(DIST)
	$(NPM)/env-cmd .env.prod $(NPM)/webpack
.PHONY: webpack

watch-webpack: $(DIST)
	$(NPM)/env-cmd .env.dev $(NPM)/webpack --watch
.PHONY: watch-webpack

css: $(DIST)
	$(NPM)/postcss \
        static/css/index.css \
        --output $(DIST)/index.css
.PHONY: css

clean-css:
	rm -Rf $(DIST)/index.css
.PHONY: clean-css

watch-css: $(DIST)
	$(NPM)/postcss \
        static/css/index.css \
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
		--watch \
		--no-ghost-mode

gh-pages: build
	git worktree remove ./gh-pages || exit 0
	git worktree add ./gh-pages gh-pages
	mv dist/* gh-pages
	cp netlify.toml ./gh-pages/