TAG ?= v1.2.0

.PHONY: release draft build check clean test

release:
	python scripts/pipeline.py $(TAG)

draft:
	python scripts/pipeline.py $(TAG) --draft

build:
	python scripts/pipeline.py $(TAG) --no-publish

check:
	cargo check --workspace
	cargo test --workspace

test:
	cargo test --workspace

clean:
	python -c "import shutil; shutil.rmtree('dist', ignore_errors=True)"
	cargo clean
