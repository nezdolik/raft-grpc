extern crate grpc_raft;
extern crate grpc;

use std::thread;

use grpc::Result;

use grpc_raft::raftmessaging_grpc::*;
use grpc_raft::raftmessaging::*;

struct RaftMessengerImpl;

impl RaftMessenger for RaftMessengerImpl {
    fn append_entries(&self, o: ::grpc::RequestOptions, p: super::raftmessaging::AppendEntriesRequest) -> Result<AppendEntriesResponse> {
        let mut resp = AppendEntriesResponse::new();
        let leaderId = p.get_leaderId();
        println!("greeting request from {}", leaderId);
        resp.set_success(true);
        Ok(resp)
    }
    fn request_vote(&self, o: ::grpc::RequestOptions, p: super::raftmessaging::RequestVoteRequest) -> ::grpc::SingleResponse<super::raftmessaging::RequestVoteResponse> {
        unimplemented!()
    }
}

fn main() {
    let _server = RaftMessengerServer::new(50051, RaftMessengerImpl);

    loop {
        thread::park();
    }
}