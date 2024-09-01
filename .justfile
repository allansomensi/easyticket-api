# Services
up:
    @docker compose -f compose.yaml up -d

down:
    @docker compose -f compose.yaml down

restart:
    @docker compose -f compose.yaml down
    @docker compose -f compose.yaml up -d

logs:
    @docker compose -f compose.yaml logs -f

# Dev utils
dev:
    just up
    @sleep 1
    just run

run:
    @cargo watch -q -c -x run

test:
    @cargo watch -q -c -x test
