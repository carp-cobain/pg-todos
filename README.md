# pg-todos

An axum web-service that manages simplistic todo lists.

The goal of this project is to benchmark [Axum](https://docs.rs/axum/latest/axum/)
using a tokio runtime per core, each with a dedicated
[tokio-postgres](https://crates.io/crates/tokio-postgres) connection.

This is a realistic setup, but does not use a dedicated connection pool like bb8, deadpool, or sqlx.
The service will fail-fast (ie abort on panic) when db connections are closed. To run this setup
in a test, staging or prod environment, some kind of supervision is required (use auto restarts w/
crashloop backoff).

## database

This demo uses `diesel` for database setup and migrations.

To install the CLI tool

```sh
cargo install diesel_cli --no-default-features --features postgres
```

To create database, and run migrations

```sh
diesel database setup
```

## seed data

After db setup, can seed the `stories` table with the project `wrk` lua script

```sh
cargo run
```

then, in another terminal

```sh
brew install wrk # if not already installed
wrk -t1 -c1 --script scripts/create-stories.lua http://127.0.0.1:8080/
```

## benchmark

```sh
cargo build --release
RUST_LOG=off target/release/pg-todos
```

then, in another terminal,

```sh
wrk -t10 -c100 -d1m --latency http://127.0.0.1:8080/stories
```
