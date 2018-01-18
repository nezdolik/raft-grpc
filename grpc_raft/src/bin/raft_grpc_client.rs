extern crate grpc_raft;
extern crate grpc;
extern crate futures;

use grpc_raft::raftmessaging_grpc::*;
use grpc_raft::raftmessaging::*;

use std::env;

fn main() {
    let client = RaftMessengerClient::new_plain("localhost", 50051, Default::default()).unwrap();

    let mut req = AppendEntriesRequest::new();
    req.set_leaderId(100);

    let resp = client.append_entries(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}