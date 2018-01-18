// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait RaftMessenger {
    fn append_entries(&self, o: ::grpc::RequestOptions, p: super::raftmessaging::AppendEntriesRequest) -> ::grpc::SingleResponse<super::raftmessaging::AppendEntriesResponse>;

    fn request_vote(&self, o: ::grpc::RequestOptions, p: super::raftmessaging::RequestVoteRequest) -> ::grpc::SingleResponse<super::raftmessaging::RequestVoteResponse>;
}

// client

pub struct RaftMessengerClient {
    grpc_client: ::grpc::Client,
    method_AppendEntries: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::raftmessaging::AppendEntriesRequest, super::raftmessaging::AppendEntriesResponse>>,
    method_RequestVote: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::raftmessaging::RequestVoteRequest, super::raftmessaging::RequestVoteResponse>>,
}

impl RaftMessengerClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        RaftMessengerClient {
            grpc_client: grpc_client,
            method_AppendEntries: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/raftmessaging.RaftMessenger/AppendEntries".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RequestVote: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/raftmessaging.RaftMessenger/RequestVote".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            RaftMessengerClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            RaftMessengerClient::with_client(c)
        })
    }
}

impl RaftMessenger for RaftMessengerClient {
    fn append_entries(&self, o: ::grpc::RequestOptions, p: super::raftmessaging::AppendEntriesRequest) -> ::grpc::SingleResponse<super::raftmessaging::AppendEntriesResponse> {
        self.grpc_client.call_unary(o, p, self.method_AppendEntries.clone())
    }

    fn request_vote(&self, o: ::grpc::RequestOptions, p: super::raftmessaging::RequestVoteRequest) -> ::grpc::SingleResponse<super::raftmessaging::RequestVoteResponse> {
        self.grpc_client.call_unary(o, p, self.method_RequestVote.clone())
    }
}

// server

pub struct RaftMessengerServer;


impl RaftMessengerServer {
    pub fn new_service_def<H : RaftMessenger + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/raftmessaging.RaftMessenger",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/raftmessaging.RaftMessenger/AppendEntries".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.append_entries(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/raftmessaging.RaftMessenger/RequestVote".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.request_vote(o, p))
                    },
                ),
            ],
        )
    }
}
