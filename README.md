# baby_schema
Keep track of your baby schedule using AWS Lambda, Rust and DynamoDb

[![build status](https://github.com/JaderDias/baby_schema/workflows/Rust/badge.svg)](https://github.com/JaderDias/baby_schema/actions?query=workflow%3ARust)
[![lint status](https://github.com/JaderDias/baby_schema/workflows/Linter/badge.svg)](https://github.com/JaderDias/baby_schema/actions?query=workflow%3ALinter)
[![dependencies status](https://github.com/JaderDias/baby_schema/workflows/Dependencies/badge.svg)](https://github.com/JaderDias/baby_schema/actions?query=workflow%3ADependencies)
[![dependency status](https://deps.rs/repo/github/JaderDias/baby_schema/status.svg)](https://deps.rs/repo/github/JaderDias/baby_schema)
## Supported development hosts

* Linux
* MacOS

## Requirements

### Development & testing

* Docker Desktop up and running
* docker-compose
* gcc
* Rust toolchain

### additional deployment requirements

* AWS Command Line Interface

### additional macOS with Apple Silicon requirements

* musl-cross with x86_64
```bash
brew install filosottile/musl-cross/musl-cross --with-x86_64
```

## Run tests locally

```bash
./run.sh
```

on another terminal

```bash
./run_test.sh
```
