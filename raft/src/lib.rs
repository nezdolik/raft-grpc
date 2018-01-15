#![feature(proc_macro, conservative_impl_trait, generators)]
#![deny(missing_docs)]
//! crate docs
#[macro_use]

extern crate log;



extern crate futures_await as futures;

use futures::prelude::*;


#[async]
fn foo() -> Result<i32> {
    Ok(1 + await!(bar())?)
}

#[async]
fn bar() -> Result<i32> {
    Ok(2)
}



pub mod term;

use std::collections::LinkedList;
use std::thread::*;

use term::*;


const N_SERVERS: usize = 2;

#[derive(Clone)]
/// Constructs a ...
enum Role {
    Unknown,
    Follower,
    Candidate,
    Leader
}

#[derive(Clone)]
/// Constructs a ...
struct Command {
    //TODO
}

#[derive(Clone)]
/// Constructs a ...
struct LogEntry {
    index: u64,
    command: Command,
    term: Term
}

/// Constructs a ...
trait Runnable {
    fn run(&mut self);
    fn stop(&mut self);
}

trait Sender<M> {
    fn send(self, msg: M);
}

/// Constructs a ...
pub struct Server {
    id: usize,
    role: Role,
    current_term: Option<Term>,
    voted_for: Option<Box<Server>>,
    log: LinkedList<LogEntry>,
    is_running: bool,
    handle: Option<JoinHandle<()>>
}

impl Server {
    /// Constructs a ...
    pub fn new(id: usize) -> Server {
        Server {
            id: id,
            role: Role::Unknown,
            current_term: None,
            voted_for: None,
            log: LinkedList::new(),
            is_running: false,
            handle: None
        }
    }

    /// Constructs a ...
    pub fn run(&self) {
        loop {
            println!("Id:{} hello world", &self.id);
        }
    }
}


impl Runnable for Server {
    fn run(&mut self) {
        if self.is_running {
            info!("Server {} already started", self.id);
            return;
        }

        let result = Builder::new().name("child1".to_string()).spawn(move || {
            println!("Thread running");
        });

        self.handle = match result {
            Ok(result) => Option::from(result),
            Err(err) =>
                panic!("Unable to allocate server thread {}", err)
        };

        self.is_running = true;
        self.role = Role::Follower;

        info!("Server {} started", self.id);

    }

    fn stop(&mut self) {
        if !self.is_running {
            info!("Server {} already stopped", self.id);
            return;
        }

        self.is_running = false;
        self.role = Role::Unknown;


        info!("Server {} stopped", self.id);
    }
}


//pub fn start() -> () {
//    for n in 0..N_SERVERS {
//        let handle = thread::spawn(move || {
//            let server = Server::new(n);
//            server.run();
//        });
//
//        //handle.join();
//    }
//}

/// Constructs a ...
pub fn start() -> () {
    for n in 0..N_SERVERS {
        print!("***called for {}", n);
        let future = start_server(n);
    }
}


#[async]
fn start_server(id: usize) -> Result<u32>{
    print!("called for {}", id);
    let server = Server::new(id);
    server.run();
    Ok(2)
}





