# ENDPOINT ?= solana.substreams.pinax.network:443
ENDPOINT ?= accounts.mainnet.sol.streamingfast.io:443

START_BLOCK ?= 345923179

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack

.PHONY: noop
noop: build
	substreams-sink-noop $(ENDPOINT) substreams.yaml map_events -H "X-Sf-Substreams-Parallel-Jobs: 600"

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK) --network solana

.PHONY: prod
prod: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_events -s 0 -t 0 --limit-processed-blocks 0 --production-mode