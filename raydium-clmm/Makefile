ENDPOINT ?= solana.substreams.pinax.network:443
START_BLOCK ?= 0
PARALLEL_JOBS ?= 500

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK) -t 0 --limit-processed-blocks 0 --network solana
