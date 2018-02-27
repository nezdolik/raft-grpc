#!/bin/bash
docker-compose stop raft-server
docker-compose rm -fv raft-server

docker-compose stop raft-client1
docker-compose rm -fv raft-client1

docker-compose stop raft-client2
docker-compose rm -fv raft-client2

