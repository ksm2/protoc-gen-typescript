#!/bin/sh
PATH=$PATH:$PWD/target/debug
mkdir -p gen
rm -f gen/*.ts
protoc \
  --proto_path=include \
  --typescript_out=gen \
  include/test.proto \
  include/timestamp.proto
