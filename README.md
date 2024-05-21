# pg-todos

An axum web-service that manages simplistic todo lists.

The goal of this project is to benchmark [Axum](https://docs.rs/axum/latest/axum/)
with [tokio-postgres](https://crates.io/crates/tokio-postgres).

This is a realistic setup, but does not use connection pooling - bb8, deadpool, sqlx, etc.
The service will fail-fast (ie abort on panic) when db connections are closed. To run this setup,
some kind of supervision is required (like auto restarts w/ crashloop backoff).

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
