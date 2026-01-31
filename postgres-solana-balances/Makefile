ENDPOINT ?= solana.substreams.pinax.network:443
START_BLOCK ?= 350000000
STOP_BLOCK ?= 350002000
PARALLEL_JOBS ?= 500

# PostgreSQL connection string
# Format: psql://<user>:<password>@<host>:<port>/<database>?sslmode=disable
PG_DSN ?= psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable

.PHONY: pack
pack:
	substreams pack

.PHONY: dev
dev:
	substreams-sink-sql run $(PG_DSN) substreams.yaml -e $(ENDPOINT) $(START_BLOCK):$(STOP_BLOCK) --undo-buffer-size 1 --on-module-hash-mismatch=warn --batch-block-flush-interval 1 --infinite-retry --development-mode

.PHONY: setup
setup: pack
	substreams-sink-sql setup $(PG_DSN) substreams.yaml
