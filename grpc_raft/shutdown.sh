#!/bin/bash
docker stop raft-server
#docker-compose rm -fv raft-server
sleep 10

docker stop raft-client1
#docker-compose rm -fv raft-client1
sleep 10

docker stop raft-client2
#docker-compose rm -fv raft-client2
sleep 10

