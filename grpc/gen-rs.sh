#!/bin/sh

die() {
    echo "$@" >&2
    exit 1
}

set -ex

# for protoc
PATH=$HOME/.cargo/bin:$PATH

protoc_ver=$(protoc --version)
case "$protoc_ver" in
    "libprotoc 3"*) ;;
    *) die "protoc version 3 required: $protoc_ver" ;;
esac

protoc --rust_out=src raftmessaging.proto
#protoc --rust-grpc_out=src raftmessaging.proto

# vim: set ts=4 sw=4 et:
