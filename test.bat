@ECHO OFF

del /F gen\*.ts

protoc ^
  --proto_path=include ^
  --plugin=protoc-gen-typescript=target\debug\protoc-gen-typescript.exe ^
  --typescript_out=gen ^
  include\api.proto ^
  include\test.proto ^
  include\duration.proto ^
  include\wrappers.proto ^
  include\timestamp.proto
