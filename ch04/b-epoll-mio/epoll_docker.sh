#!/bin/bash
set -e

docker compose build   # bruker cache og er rask hvis ingenting har endret seg
docker compose up --abort-on-container-exit --exit-code-from epoll
docker compose down