.PHONY: default

default:
	cargo fmt
	make -C docker/reopenconnect

