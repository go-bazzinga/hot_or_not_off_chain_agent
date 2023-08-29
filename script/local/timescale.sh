#!/usr/bin/env bash
set -euo pipefail

docker run --detach --name timescaledb --publish 5432:5432 --env POSTGRES_PASSWORD=password --rm timescale/timescaledb:latest-pg15

# sqlx migrate run