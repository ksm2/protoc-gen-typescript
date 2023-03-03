#!/bin/sh
PATH=$PATH:$PWD/target/debug
mkdir -p gen
rm -f gen/*.tf
protoc \
  --proto_path=example \
  --typescript_out=gen \
  example/test.proto
