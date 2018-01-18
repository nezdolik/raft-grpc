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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct RequestVoteRequest {
    // message fields
    pub term: u64,
    pub leaderId: u64,
    pub lastLogIndex: u64,
    pub lastLogTerm: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestVoteRequest {}

impl RequestVoteRequest {
    pub fn new() -> RequestVoteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestVoteRequest {
        static mut instance: ::protobuf::lazy::Lazy<RequestVoteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestVoteRequest,
        };
        unsafe {
            instance.get(RequestVoteRequest::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // uint64 leaderId = 2;

    pub fn clear_leaderId(&mut self) {
        self.leaderId = 0;
    }

    // Param is passed by value, moved
    pub fn set_leaderId(&mut self, v: u64) {
        self.leaderId = v;
    }

    pub fn get_leaderId(&self) -> u64 {
        self.leaderId
    }

    fn get_leaderId_for_reflect(&self) -> &u64 {
        &self.leaderId
    }

    fn mut_leaderId_for_reflect(&mut self) -> &mut u64 {
        &mut self.leaderId
    }

    // uint64 lastLogIndex = 3;

    pub fn clear_lastLogIndex(&mut self) {
        self.lastLogIndex = 0;
    }

    // Param is passed by value, moved
    pub fn set_lastLogIndex(&mut self, v: u64) {
        self.lastLogIndex = v;
    }

    pub fn get_lastLogIndex(&self) -> u64 {
        self.lastLogIndex
    }

    fn get_lastLogIndex_for_reflect(&self) -> &u64 {
        &self.lastLogIndex
    }

    fn mut_lastLogIndex_for_reflect(&mut self) -> &mut u64 {
        &mut self.lastLogIndex
    }

    // uint64 lastLogTerm = 4;

    pub fn clear_lastLogTerm(&mut self) {
        self.lastLogTerm = 0;
    }

    // Param is passed by value, moved
    pub fn set_lastLogTerm(&mut self, v: u64) {
        self.lastLogTerm = v;
    }

    pub fn get_lastLogTerm(&self) -> u64 {
        self.lastLogTerm
    }

    fn get_lastLogTerm_for_reflect(&self) -> &u64 {
        &self.lastLogTerm
    }

    fn mut_lastLogTerm_for_reflect(&mut self) -> &mut u64 {
        &mut self.lastLogTerm
    }
}

impl ::protobuf::Message for RequestVoteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.leaderId = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastLogIndex = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastLogTerm = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.leaderId != 0 {
            my_size += ::protobuf::rt::value_size(2, self.leaderId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lastLogIndex != 0 {
            my_size += ::protobuf::rt::value_size(3, self.lastLogIndex, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lastLogTerm != 0 {
            my_size += ::protobuf::rt::value_size(4, self.lastLogTerm, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.leaderId != 0 {
            os.write_uint64(2, self.leaderId)?;
        }
        if self.lastLogIndex != 0 {
            os.write_uint64(3, self.lastLogIndex)?;
        }
        if self.lastLogTerm != 0 {
            os.write_uint64(4, self.lastLogTerm)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestVoteRequest {
    fn new() -> RequestVoteRequest {
        RequestVoteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestVoteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    RequestVoteRequest::get_term_for_reflect,
                    RequestVoteRequest::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "leaderId",
                    RequestVoteRequest::get_leaderId_for_reflect,
                    RequestVoteRequest::mut_leaderId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastLogIndex",
                    RequestVoteRequest::get_lastLogIndex_for_reflect,
                    RequestVoteRequest::mut_lastLogIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastLogTerm",
                    RequestVoteRequest::get_lastLogTerm_for_reflect,
                    RequestVoteRequest::mut_lastLogTerm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestVoteRequest>(
                    "RequestVoteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestVoteRequest {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_leaderId();
        self.clear_lastLogIndex();
        self.clear_lastLogTerm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestVoteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestVoteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AppendEntriesRequest {
    // message fields
    pub term: u64,
    pub leaderId: u64,
    pub prevLogIndex: u64,
    pub prevLogTerm: u64,
    pub entries: ::protobuf::RepeatedField<LogEntry>,
    pub leaderCommit: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntriesRequest {}

impl AppendEntriesRequest {
    pub fn new() -> AppendEntriesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntriesRequest {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntriesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntriesRequest,
        };
        unsafe {
            instance.get(AppendEntriesRequest::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // uint64 leaderId = 2;

    pub fn clear_leaderId(&mut self) {
        self.leaderId = 0;
    }

    // Param is passed by value, moved
    pub fn set_leaderId(&mut self, v: u64) {
        self.leaderId = v;
    }

    pub fn get_leaderId(&self) -> u64 {
        self.leaderId
    }

    fn get_leaderId_for_reflect(&self) -> &u64 {
        &self.leaderId
    }

    fn mut_leaderId_for_reflect(&mut self) -> &mut u64 {
        &mut self.leaderId
    }

    // uint64 prevLogIndex = 3;

    pub fn clear_prevLogIndex(&mut self) {
        self.prevLogIndex = 0;
    }

    // Param is passed by value, moved
    pub fn set_prevLogIndex(&mut self, v: u64) {
        self.prevLogIndex = v;
    }

    pub fn get_prevLogIndex(&self) -> u64 {
        self.prevLogIndex
    }

    fn get_prevLogIndex_for_reflect(&self) -> &u64 {
        &self.prevLogIndex
    }

    fn mut_prevLogIndex_for_reflect(&mut self) -> &mut u64 {
        &mut self.prevLogIndex
    }

    // uint64 prevLogTerm = 4;

    pub fn clear_prevLogTerm(&mut self) {
        self.prevLogTerm = 0;
    }

    // Param is passed by value, moved
    pub fn set_prevLogTerm(&mut self, v: u64) {
        self.prevLogTerm = v;
    }

    pub fn get_prevLogTerm(&self) -> u64 {
        self.prevLogTerm
    }

    fn get_prevLogTerm_for_reflect(&self) -> &u64 {
        &self.prevLogTerm
    }

    fn mut_prevLogTerm_for_reflect(&mut self) -> &mut u64 {
        &mut self.prevLogTerm
    }

    // repeated .raftmessaging.LogEntry entries = 5;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<LogEntry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<LogEntry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<LogEntry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[LogEntry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<LogEntry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LogEntry> {
        &mut self.entries
    }

    // uint64 leaderCommit = 6;

    pub fn clear_leaderCommit(&mut self) {
        self.leaderCommit = 0;
    }

    // Param is passed by value, moved
    pub fn set_leaderCommit(&mut self, v: u64) {
        self.leaderCommit = v;
    }

    pub fn get_leaderCommit(&self) -> u64 {
        self.leaderCommit
    }

    fn get_leaderCommit_for_reflect(&self) -> &u64 {
        &self.leaderCommit
    }

    fn mut_leaderCommit_for_reflect(&mut self) -> &mut u64 {
        &mut self.leaderCommit
    }
}

impl ::protobuf::Message for AppendEntriesRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.leaderId = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.prevLogIndex = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.prevLogTerm = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.leaderCommit = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.leaderId != 0 {
            my_size += ::protobuf::rt::value_size(2, self.leaderId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.prevLogIndex != 0 {
            my_size += ::protobuf::rt::value_size(3, self.prevLogIndex, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.prevLogTerm != 0 {
            my_size += ::protobuf::rt::value_size(4, self.prevLogTerm, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.leaderCommit != 0 {
            my_size += ::protobuf::rt::value_size(6, self.leaderCommit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.leaderId != 0 {
            os.write_uint64(2, self.leaderId)?;
        }
        if self.prevLogIndex != 0 {
            os.write_uint64(3, self.prevLogIndex)?;
        }
        if self.prevLogTerm != 0 {
            os.write_uint64(4, self.prevLogTerm)?;
        }
        for v in &self.entries {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.leaderCommit != 0 {
            os.write_uint64(6, self.leaderCommit)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendEntriesRequest {
    fn new() -> AppendEntriesRequest {
        AppendEntriesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntriesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    AppendEntriesRequest::get_term_for_reflect,
                    AppendEntriesRequest::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "leaderId",
                    AppendEntriesRequest::get_leaderId_for_reflect,
                    AppendEntriesRequest::mut_leaderId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "prevLogIndex",
                    AppendEntriesRequest::get_prevLogIndex_for_reflect,
                    AppendEntriesRequest::mut_prevLogIndex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "prevLogTerm",
                    AppendEntriesRequest::get_prevLogTerm_for_reflect,
                    AppendEntriesRequest::mut_prevLogTerm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LogEntry>>(
                    "entries",
                    AppendEntriesRequest::get_entries_for_reflect,
                    AppendEntriesRequest::mut_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "leaderCommit",
                    AppendEntriesRequest::get_leaderCommit_for_reflect,
                    AppendEntriesRequest::mut_leaderCommit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntriesRequest>(
                    "AppendEntriesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntriesRequest {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_leaderId();
        self.clear_prevLogIndex();
        self.clear_prevLogTerm();
        self.clear_entries();
        self.clear_leaderCommit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AppendEntriesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AppendEntriesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestVoteResponse {
    // message fields
    pub term: u64,
    pub voteGranted: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestVoteResponse {}

impl RequestVoteResponse {
    pub fn new() -> RequestVoteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestVoteResponse {
        static mut instance: ::protobuf::lazy::Lazy<RequestVoteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestVoteResponse,
        };
        unsafe {
            instance.get(RequestVoteResponse::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // bool voteGranted = 2;

    pub fn clear_voteGranted(&mut self) {
        self.voteGranted = false;
    }

    // Param is passed by value, moved
    pub fn set_voteGranted(&mut self, v: bool) {
        self.voteGranted = v;
    }

    pub fn get_voteGranted(&self) -> bool {
        self.voteGranted
    }

    fn get_voteGranted_for_reflect(&self) -> &bool {
        &self.voteGranted
    }

    fn mut_voteGranted_for_reflect(&mut self) -> &mut bool {
        &mut self.voteGranted
    }
}

impl ::protobuf::Message for RequestVoteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.voteGranted = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.voteGranted != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.voteGranted != false {
            os.write_bool(2, self.voteGranted)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestVoteResponse {
    fn new() -> RequestVoteResponse {
        RequestVoteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestVoteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    RequestVoteResponse::get_term_for_reflect,
                    RequestVoteResponse::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "voteGranted",
                    RequestVoteResponse::get_voteGranted_for_reflect,
                    RequestVoteResponse::mut_voteGranted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestVoteResponse>(
                    "RequestVoteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestVoteResponse {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_voteGranted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestVoteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestVoteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AppendEntriesResponse {
    // message fields
    pub term: u64,
    pub success: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEntriesResponse {}

impl AppendEntriesResponse {
    pub fn new() -> AppendEntriesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEntriesResponse {
        static mut instance: ::protobuf::lazy::Lazy<AppendEntriesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEntriesResponse,
        };
        unsafe {
            instance.get(AppendEntriesResponse::new)
        }
    }

    // uint64 term = 1;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }

    // bool success = 2;

    pub fn clear_success(&mut self) {
        self.success = false;
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = v;
    }

    pub fn get_success(&self) -> bool {
        self.success
    }

    fn get_success_for_reflect(&self) -> &bool {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut bool {
        &mut self.success
    }
}

impl ::protobuf::Message for AppendEntriesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.success = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(1, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.success != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.term != 0 {
            os.write_uint64(1, self.term)?;
        }
        if self.success != false {
            os.write_bool(2, self.success)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendEntriesResponse {
    fn new() -> AppendEntriesResponse {
        AppendEntriesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEntriesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    AppendEntriesResponse::get_term_for_reflect,
                    AppendEntriesResponse::mut_term_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    AppendEntriesResponse::get_success_for_reflect,
                    AppendEntriesResponse::mut_success_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEntriesResponse>(
                    "AppendEntriesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEntriesResponse {
    fn clear(&mut self) {
        self.clear_term();
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AppendEntriesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AppendEntriesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LogEntry {
    // message fields
    pub buff: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LogEntry {}

impl LogEntry {
    pub fn new() -> LogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogEntry {
        static mut instance: ::protobuf::lazy::Lazy<LogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogEntry,
        };
        unsafe {
            instance.get(LogEntry::new)
        }
    }

    // bytes buff = 2;

    pub fn clear_buff(&mut self) {
        self.buff.clear();
    }

    // Param is passed by value, moved
    pub fn set_buff(&mut self, v: ::std::vec::Vec<u8>) {
        self.buff = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buff(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.buff
    }

    // Take field
    pub fn take_buff(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.buff, ::std::vec::Vec::new())
    }

    pub fn get_buff(&self) -> &[u8] {
        &self.buff
    }

    fn get_buff_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.buff
    }

    fn mut_buff_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.buff
    }
}

impl ::protobuf::Message for LogEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.buff)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.buff.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.buff);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.buff.is_empty() {
            os.write_bytes(2, &self.buff)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogEntry {
    fn new() -> LogEntry {
        LogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "buff",
                    LogEntry::get_buff_for_reflect,
                    LogEntry::mut_buff_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogEntry>(
                    "LogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogEntry {
    fn clear(&mut self) {
        self.clear_buff();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13raftmessaging.proto\x12\rraftmessaging\"\x8a\x01\n\x12RequestVoteR\
    equest\x12\x12\n\x04term\x18\x01\x20\x01(\x04R\x04term\x12\x1a\n\x08lead\
    erId\x18\x02\x20\x01(\x04R\x08leaderId\x12\"\n\x0clastLogIndex\x18\x03\
    \x20\x01(\x04R\x0clastLogIndex\x12\x20\n\x0blastLogTerm\x18\x04\x20\x01(\
    \x04R\x0blastLogTerm\"\xe3\x01\n\x14AppendEntriesRequest\x12\x12\n\x04te\
    rm\x18\x01\x20\x01(\x04R\x04term\x12\x1a\n\x08leaderId\x18\x02\x20\x01(\
    \x04R\x08leaderId\x12\"\n\x0cprevLogIndex\x18\x03\x20\x01(\x04R\x0cprevL\
    ogIndex\x12\x20\n\x0bprevLogTerm\x18\x04\x20\x01(\x04R\x0bprevLogTerm\
    \x121\n\x07entries\x18\x05\x20\x03(\x0b2\x17.raftmessaging.LogEntryR\x07\
    entries\x12\"\n\x0cleaderCommit\x18\x06\x20\x01(\x04R\x0cleaderCommit\"K\
    \n\x13RequestVoteResponse\x12\x12\n\x04term\x18\x01\x20\x01(\x04R\x04ter\
    m\x12\x20\n\x0bvoteGranted\x18\x02\x20\x01(\x08R\x0bvoteGranted\"E\n\x15\
    AppendEntriesResponse\x12\x12\n\x04term\x18\x01\x20\x01(\x04R\x04term\
    \x12\x18\n\x07success\x18\x02\x20\x01(\x08R\x07success\"\x1e\n\x08LogEnt\
    ry\x12\x12\n\x04buff\x18\x02\x20\x01(\x0cR\x04buff2\xc5\x01\n\rRaftMesse\
    nger\x12\\\n\rAppendEntries\x12#.raftmessaging.AppendEntriesRequest\x1a$\
    .raftmessaging.AppendEntriesResponse\"\0\x12V\n\x0bRequestVote\x12!.raft\
    messaging.RequestVoteRequest\x1a\".raftmessaging.RequestVoteResponse\"\0\
    J\xba\x0b\n\x06\x12\x04\0\0.\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\
    \x01\x02\x12\x03\x03\x08\x15\n\n\n\x02\x06\0\x12\x04\x05\0\x0b\x01\n\n\n\
    \x03\x06\0\x01\x12\x03\x05\x08\x15\n\x0c\n\x04\x06\0\x02\0\x12\x04\x06\
    \x04\x07\x05\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x06\x08\x15\n\x0c\n\x05\
    \x06\0\x02\0\x02\x12\x03\x06\x17+\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\
    \x066K\n\x0c\n\x04\x06\0\x02\x01\x12\x04\t\x04\n\x05\n\x0c\n\x05\x06\0\
    \x02\x01\x01\x12\x03\t\x08\x13\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\t\
    \x15'\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\t2E\n3\n\x02\x04\0\x12\x04\
    \x0e\0\x13\x01\x1a'\x20Invoked\x20by\x20candidates\x20to\x20gather\x20vo\
    tes\n\n\n\n\x03\x04\0\x01\x12\x03\x0e\x08\x1a\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x0f\x04\x14\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0f\x04\x0e\x1c\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0f\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x0f\x0b\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0f\x12\x13\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x10\x04\x18\n\r\n\x05\x04\0\x02\x01\x04\
    \x12\x04\x10\x04\x0f\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x10\x04\n\
    \n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x10\x0b\x13\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x10\x16\x17\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x11\x04\
    \x1c\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x11\x04\x10\x18\n\x0c\n\x05\x04\
    \0\x02\x02\x05\x12\x03\x11\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x11\x0b\x17\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x11\x1a\x1b\n\x0b\n\
    \x04\x04\0\x02\x03\x12\x03\x12\x04\x1b\n\r\n\x05\x04\0\x02\x03\x04\x12\
    \x04\x12\x04\x11\x1c\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x12\x04\n\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x12\x0b\x16\n\x0c\n\x05\x04\0\x02\
    \x03\x03\x12\x03\x12\x19\x1a\nO\n\x02\x04\x01\x12\x04\x16\0\x1e\x01\x1aC\
    \x20Invoked\x20by\x20leader\x20to\x20replicate\x20log\x20entries;\x20als\
    o\x20used\x20as\x20hearbeat\n\n\n\n\x03\x04\x01\x01\x12\x03\x16\x08\x1c\
    \n\x0b\n\x04\x04\x01\x02\0\x12\x03\x17\x04\x14\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04\x17\x04\x16\x1e\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x17\
    \x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x17\x0b\x0f\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x17\x12\x13\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\
    \x18\x04\x18\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x18\x04\x17\x14\n\x0c\
    \n\x05\x04\x01\x02\x01\x05\x12\x03\x18\x04\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03\x18\x0b\x13\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x18\x16\
    \x17\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x19\x04\x1c\n\r\n\x05\x04\x01\
    \x02\x02\x04\x12\x04\x19\x04\x18\x18\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\
    \x03\x19\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x19\x0b\x17\n\x0c\
    \n\x05\x04\x01\x02\x02\x03\x12\x03\x19\x1a\x1b\n\x0b\n\x04\x04\x01\x02\
    \x03\x12\x03\x1a\x04\x1b\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\x1a\x04\
    \x19\x1c\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x1a\x04\n\n\x0c\n\x05\
    \x04\x01\x02\x03\x01\x12\x03\x1a\x0b\x16\n\x0c\n\x05\x04\x01\x02\x03\x03\
    \x12\x03\x1a\x19\x1a\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x1b\x04\"\n\x0c\
    \n\x05\x04\x01\x02\x04\x04\x12\x03\x1b\x04\x0c\n\x0c\n\x05\x04\x01\x02\
    \x04\x06\x12\x03\x1b\r\x15\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\x1b\
    \x16\x1d\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x1b\x20!\n\x0b\n\x04\
    \x04\x01\x02\x05\x12\x03\x1c\x04\x1c\n\r\n\x05\x04\x01\x02\x05\x04\x12\
    \x04\x1c\x04\x1b\"\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x03\x1c\x04\n\n\
    \x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\x1c\x0b\x17\n\x0c\n\x05\x04\x01\
    \x02\x05\x03\x12\x03\x1c\x1a\x1b\n\n\n\x02\x04\x02\x12\x04\x20\0#\x01\n\
    \n\n\x03\x04\x02\x01\x12\x03\x20\x08\x1b\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03!\x04\x14\n\r\n\x05\x04\x02\x02\0\x04\x12\x04!\x04\x20\x1d\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03!\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03!\x0b\x0f\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03!\x12\x13\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\"\x04\x19\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\
    \"\x04!\x14\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\"\x04\x08\n\x0c\n\
    \x05\x04\x02\x02\x01\x01\x12\x03\"\t\x14\n\x0c\n\x05\x04\x02\x02\x01\x03\
    \x12\x03\"\x17\x18\n\n\n\x02\x04\x03\x12\x04%\0(\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03%\x08\x1d\n\x0b\n\x04\x04\x03\x02\0\x12\x03&\x04\x14\n\r\n\
    \x05\x04\x03\x02\0\x04\x12\x04&\x04%\x1f\n\x0c\n\x05\x04\x03\x02\0\x05\
    \x12\x03&\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03&\x0b\x0f\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x03&\x12\x13\n\x0b\n\x04\x04\x03\x02\x01\x12\
    \x03'\x04\x15\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04'\x04&\x14\n\x0c\n\
    \x05\x04\x03\x02\x01\x05\x12\x03'\x04\x08\n\x0c\n\x05\x04\x03\x02\x01\
    \x01\x12\x03'\t\x10\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03'\x13\x14\n\n\
    \n\x02\x04\x04\x12\x04+\0.\x01\n\n\n\x03\x04\x04\x01\x12\x03+\x08\x10\n\
    \"\n\x04\x04\x04\x02\0\x12\x03-\x04\x13\x1a\x15TODO\x20uint64\x20key\x20\
    =\x201;\n\n\r\n\x05\x04\x04\x02\0\x04\x12\x04-\x04+\x12\n\x0c\n\x05\x04\
    \x04\x02\0\x05\x12\x03-\x04\t\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03-\n\
    \x0e\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03-\x11\x12b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
