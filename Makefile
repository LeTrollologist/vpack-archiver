TAG ?= v2.0.0

.PHONY: release draft build check clean test gui

release:
	python scripts/pipeline.py $(TAG)

draft:
	python scripts/pipeline.py $(TAG) --draft

build:
	python scripts/pipeline.py $(TAG) --no-publish

gui:
	cargo build --release -p vpack-gui

check:
	cargo check --workspace
	cargo test --workspace

test:
	cargo test --workspace

clean:
	python -c "import shutil; shutil.rmtree('dist', ignore_errors=True)"
	cargo clean
