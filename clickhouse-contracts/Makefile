.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: gui
gui: build
	substreams gui substreams.yaml db_out -e eth.substreams.pinax.network:443 -s 10000000 -t +1000 --limit-processed-blocks 0

.PHONY: dev
dev: build
	substreams-sink-sql run clickhouse://default:default@localhost:9000/test substreams.yaml -e eth.substreams.pinax.network:443 10000000:10010000 --undo-buffer-size 1 --batch-block-flush-interval 1

.PHONY: setup
setup: build
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/test substreams.yaml
