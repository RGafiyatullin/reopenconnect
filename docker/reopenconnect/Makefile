.PHONY: default up up-build up-run dn build run-shell

FEATURES = --no-default-features --features with-tokio-tun

default: build-debug build-examples

up: up-run up-build

up-build:
	docker compose up -d build

up-run:
	docker compose up -d run

dn:
	docker compose down

build-debug: up-build
	docker compose exec build cargo build $(FEATURES)

build-examples: up-build
	docker compose exec build cargo build $(FEATURES) --examples

build-release: up-build
	docker compose exec build cargo build $(FEATURES) --release

run-shell: up-run
	docker compose exec run /bin/bash
