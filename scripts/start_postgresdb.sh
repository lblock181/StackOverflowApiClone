#! /bin/bash
POSTGRES_PASSWORD=$(< ./devonly/dbpw)
docker run --name stackoverflowdb -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -p 5508:5432 -d postgres