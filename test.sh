#!/bin/sh
PATH=$PATH:$PWD/target/debug
mkdir -p gen
rm -f gen/*.tf
protoc \
  --proto_path=include \
  --typescript_out=gen \
  include/*.proto
