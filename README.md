protoc-gen-typescript
=====================

A code generator for TypeScript based on Protobuf definitions, written in Rust.


## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)


## Features

- Generates well-formatted, easy to read and extendable TypeScript code
- Generated code only depends on [`google-protobuf`](https://www.npmjs.com/package/google-protobuf)
- Supports all field types
- Generates service stubs
- Generates enums
- Allows using `String`, `Number` or `BigInt` for 64-bit integers
- Enhanced integrations for `google.protobuf.Timestamp` and `google.protobuf.Duration` with JavaScript `Date`
- Support for packed and unpacked fields
- Generated code is fully tested using Jest


## Installation

Install the application using [Cargo]:

    cargo install --path .

This will place the `protoc-gen-typescript` binary by default in your `~/.cargo/bin` folder.
If you have that folder inside your path, you should be able to find it using 

    which protoc-gen-typescript


## Usage

Using [Protoc] you can now convert your Protobuf definitions into TypeScript: 

    mkdir -p gen
    protoc --typescript_out=gen --proto_path=include include/*.proto


[Cargo]: https://doc.rust-lang.org/cargo/
[Protoc]: https://grpc.io/docs/protoc-installation/
