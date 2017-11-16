pub mod term;

use std::collections::LinkedList;

use term::*;

enum Role {
    Follower,
    Candidate,
    Leader
}

struct Command{
    //TODO
}

struct LogEntry {
    index: u64,
    command: Command,
    term: Term
}

pub struct Server {
    role: Role,
    currentTerm: Option<Term>,
    votedFor: Option<Box<Server>>,
    log: LinkedList<LogEntry>

}

impl Server {
    pub fn new() -> Server {
        Server {
            role: Role::Follower,
            currentTerm: None,
            votedFor: None,
            log: LinkedList::new()
        }
    }

    pub fn run(&self) {
        println!("hello world");
    }
}

pub fn start() -> () {
    let server = Server::new();
    server.run();
}

fn main(){
    start();
}

