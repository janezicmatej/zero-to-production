#!/usr/bin/env bash

until pg_isready -h "${DB_HOST}" -p "${DB_PORT}" -U "${DB_USER}" 2>/dev/null; do
	echo "waiting for postgres"
	sleep 1
done

export TERM=xterm-256color

exec cargo watch -w ./tests -w ./src -w Cargo.toml -x check -x test -x run 
