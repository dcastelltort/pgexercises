#!/bin/sh
docker run -p 5432:5432 --name local-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
docker run -it --rm --link local-postgres:postgres -v "$(pwd)":/data postgres psql -h postgres -U postgres -f /data/clubdata.sql -d postgres -x -q
export DATABASE_URL=postgres://postgres:mysecretpassword@localhost/exercises
