#!/bin/sh
PATH=$PWD/target/debug:$PATH
mkdir -p gen
rm -f gen/*.ts
protoc \
  --proto_path=include \
  --typescript_out=gen \
  include/api.proto \
  include/test.proto \
  include/duration.proto \
  include/timestamp.proto \
  include/wrappers.proto
