#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: 'psql' is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: 'sqlx' is not installed."
    echo >&2 "Install it with 'cargo install --version='~0.6' sqlx-cli --no-default-features --features rusttls,postgres'"
    exit 1
fi





#Checj if a custom user has been set , if not default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"

#Check if a custom password has been set , if not default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

#Check if a custom database name has been set , if not default to 'postgres'

DB_NAME="${POSTGRES_DB:=newsletter}"

#Check if a custom port has been set , if not default to '5432'

DB_PORT="${POSTGRES_PORT:=5432}"

#Check if a custom host has been set , if not default to 'localhost'

DB_HOST="${POSTGRES_HOST:=localhost}"


#Launch Docker container with postgres image

docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -e POSTGRES_PORT=${DB_PORT} \
    -e POSTGRES_HOST=${DB_HOST} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000

# Keep pinging PostGres until its ready to accept commands

until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c  '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done
&2 echo "Postgres is up and running on port ${DB_PORT}!"






DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL

sqlx database create

