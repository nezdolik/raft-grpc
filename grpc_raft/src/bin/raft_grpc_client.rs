extern crate grpc_raft;
extern crate grpc;
extern crate futures;
extern crate scheduled_executor;
#[macro_use]
extern crate structopt;

use grpc_raft::raftmessaging_grpc::*;
use grpc_raft::raftmessaging::*;

use scheduled_executor::ThreadPoolExecutor;

use std::time::Duration;
use std::thread;

use structopt::StructOpt;

const  DEFAULT_GRPC_PORT: u16 = 50051;

#[derive(StructOpt, Debug)]
#[structopt(name = "raftclient")]
struct Opt {
    /// Grpc port to bind to
    #[structopt(short = "p", long = "grpcport")]
    grpcport: Option<u16>,
    /// LeaderId for raft peer
    #[structopt(short = "lid", long = "leaderId")]
    leaderid: Option<u64>,
}

fn main() {

    let args = Opt::from_args();
    println!("{:?}", args);
    let leader_id = args.leaderid.unwrap_or(100);
    let grpc_port = args.grpcport.unwrap_or(DEFAULT_GRPC_PORT);

    let client = RaftMessengerClient::new_plain("localhost", grpc_port, Default::default()).unwrap();

    let executor = ThreadPoolExecutor::new(1).expect("Thread pool creation failed");;

    executor.schedule_fixed_rate(
        Duration::from_secs(2),  // Wait 2 seconds before scheduling the first task
        Duration::from_secs(10),  // and schedule every following task at 5 seconds intervals
        move |_| {
            let mut req = AppendEntriesRequest::new();
            req.set_leaderId(leader_id);

            let resp = client.append_entries(grpc::RequestOptions::new(), req);

            println!("{:?}", resp.wait());
        },
    );

    loop {
        thread::park();
    }


}