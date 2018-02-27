extern crate grpc_raft;
extern crate grpc;
#[macro_use]
extern crate structopt;

use std::thread;

use grpc_raft::raftmessaging_grpc::*;
use grpc_raft::raftmessaging::*;

use structopt::StructOpt;


const  DEFAULT_GRPC_PORT: u16 = 50051;
const  DEFAULT_THREADPOOL_SIZE: usize = 4;

#[derive(StructOpt, Debug)]
#[structopt(name = "raftserver")]
struct Opt {
    /// Grpc port for raft server
    #[structopt(short = "p", long = "grpcport")]
    grpcport: Option<u16>,
    /// CPU pool size for raft server
    #[structopt(short = "n", long = "nthreads")]
    nthreads: Option<usize>,
}

struct RaftMessengerImpl;

impl RaftMessenger for RaftMessengerImpl {
    fn append_entries(&self, o: ::grpc::RequestOptions, p: AppendEntriesRequest) -> grpc::SingleResponse<AppendEntriesResponse> {
        let mut resp = AppendEntriesResponse::new();
        let leader_id = p.get_leaderId();
        println!("greeting request from {}", leader_id);
        resp.set_success(true);
        grpc::SingleResponse::completed(resp)
    }
    fn request_vote(&self, o: ::grpc::RequestOptions, p: RequestVoteRequest) -> grpc::SingleResponse<RequestVoteResponse> {
        unimplemented!()
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    let args = Opt::from_args();
    println!("{:?}", args);
    let grpc_port = args.grpcport.unwrap_or(DEFAULT_GRPC_PORT);
    let n_threads = args.nthreads.unwrap_or(DEFAULT_THREADPOOL_SIZE);
    server.http.set_port(grpc_port);
    server.add_service(RaftMessengerServer::new_service_def(RaftMessengerImpl));
    server.http.set_cpu_pool_threads(n_threads);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}