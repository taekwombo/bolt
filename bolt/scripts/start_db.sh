#!/bin/bash

if podman ps -a --format '{{.Names}}' | grep -w 'neo3.5'; then
    podman start neo3.5 -a
else
    podman run --name=neo3.5 --publish 7687:7687 --publish 7473:7474 --env=NEO4J_AUTH=neo4j/bolt-rs neo4j:3.5
fi 

