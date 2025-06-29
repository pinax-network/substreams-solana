ENDPOINT ?= solana.substreams.pinax.network:443
START_BLOCK ?= 349586105
PARALLEL_JOBS ?= 500

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack

.PHONY: noop
noop: build
	substreams-sink-noop $(ENDPOINT) substreams.yaml map_events -H "X-Sf-Substreams-Parallel-Jobs: $(PARALLEL_JOBS)" $(START_BLOCK):

.PHONY: gui
gui: build
	substreams run -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK) -t +10 --network solana

.PHONY: prod
prod: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_events -s $(START_BLOCK) -t 0 --limit-processed-blocks 0 --production-mode  -H "X-Sf-Substreams-Parallel-Jobs: $(PARALLEL_JOBS)"

.PHONY: index
index:
	substreams gui -e $(ENDPOINT) https://github.com/streamingfast/substreams-foundational-modules/releases/download/substreams-v0.3.3/solana-common-v0.3.3.spkg program_ids_without_votes -s $(START_BLOCK) -t 0 --limit-processed-blocks 0 --production-mode  -H "X-Sf-Substreams-Parallel-Jobs: $(PARALLEL_JOBS)"