# raft-grpc
0. May need to update Rust compiler
```cargo update rustc```
1. Generate grpc stubs
```./scripts/gen-rs.sh```
2. Build application
```grpc_raft && cargo build```
3. Start cluster
```docker-compose up```

