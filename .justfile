# Services
services-up:
    @docker compose -f compose.yaml up -d

services-down:
    @docker compose -f compose.yaml down

services-stop:
    @docker compose -f compose.yaml stop

services-restart:
    @docker compose -f compose.yaml down
    @docker compose -f compose.yaml up -d

# Migrations
migrate-create migration_name:
    @cargo sqlx migrate add {{ migration_name }}

migrate-run:
    @cargo sqlx migrate run

migrate-down:
    @cargo sqlx migrate revert

# Dev utils
dev:
    @just services-up
    @sleep 1
    @just run-watch

run-watch:
    @cargo watch -q -c -x run

test-watch:
    @cargo watch -q -c -x test

lint:
    @cargo fmt --check
    @cargo clippy

lint-fix:
    @cargo fmt
    @cargo clippy
