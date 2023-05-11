#!/usr/bin/env bash

# until pg_isready -h "${DB_HOST}" -p "${DB_PORT}" -U "${DB_USER}" 2>/dev/null; do
# 	echo "waiting for postgres"
# 	sleep 1
# done

exec cargo watch -w ./src -x check -x test -x run 
