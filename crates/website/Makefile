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

build: clean css cargo optimize webpack
.PHONY: build

clean:
	rm -Rf dist build
	mkdir -p dist build
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
	mkdir -p ./build
	mv ../../target/wasm32-unknown-unknown/release/website.* ./build
.PHONY: cargo

optimize:
	wasm-gc ./build/website.wasm ./build/website.wasm
	wasm-opt --output ./build/website.wasm -Oz ./build/website.wasm
.PHONY: optimize

clean-cargo:
	rm -Rf ../../target/wasm32-unknown-unknown
.PHONY: clean-cargo

full-clean-cargo:
	rm -Rf ../../target
.PHONY: full-clean-cargo

watch-cargo:
	cargo watch \
		-w src \
		-s "make cargo"
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
