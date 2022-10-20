# Rust Rocket MongoDB

Build a REST API with Rust, Rocket and MongoDB

[![dependency status](https://deps.rs/repo/github/yandeu/rust-rocket-mongodb/status.svg)](https://deps.rs/repo/github/yandeu/rust-rocket-mongodb)
[![CI](https://github.com/yandeu/rust-rocket-mongodb/actions/workflows/main.yml/badge.svg)](https://github.com/yandeu/rust-rocket-mongodb/actions/workflows/main.yml)

## Development

### Start MongoDB with docker

```console
docker run -d -p 27017:27017 --name mongodb -e MONGO_INITDB_ROOT_USERNAME=yannick -e MONGO_INITDB_ROOT_PASSWORD=123456 mongo:6
```

### Start MongoDB with docker (alternative)

```console
docker run -d -p 27017:27017 --name mongodb mongo:6
docker exec -it <CONTAINER_NAME> bash
use admin
db.createUser({user:"yannick", pwd: "123456", roles: ["userAdminAnyDatabase"] })
```

### Start server (and watch for changes)

```console
cargo build; cargo watch -q -c -x run
```

## Testing

```console
# windows
cargo build; .\test\test.ps1
# linux
cargo build && chmod +x test/test.sh  && ./test/test.sh
```

## Docker Build

```console
docker build -t my-rust-app .
docker run -it --rm --name my-running-app my-rust-app
```

## Other examples

- This repository [Rocket Mongo](https://github.com/marirs/rocket-mongo) is a good example as well.
- Read this [article](https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5).
