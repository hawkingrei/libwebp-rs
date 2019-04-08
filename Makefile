SHELL := /bin/bash

.SUFFIXES:

init:
	@./hack/init.sh

test: init
	@cargo test
	cargo test --release
