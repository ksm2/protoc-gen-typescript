@ECHO OFF

protoc ^
  --proto_path=example ^
  --plugin=protoc-gen-typescript=target\debug\protoc-gen-typescript.exe ^
  --typescript_out=out ^
  example\test.proto
