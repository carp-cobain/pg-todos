# pg-todos

An axum web-service that manages simplistic todo lists.

The goal of this project is to benchmark [Axum](https://docs.rs/axum/latest/axum/)
with [tokio-postgres](https://crates.io/crates/tokio-postgres).

This is a realistic setup, but does not use connection pooling - no deadpool, sqlx, etc.
The service will fail-fast (ie abort on panic) when db connections are closed.

## database

Install the `diesel` migration tool

```sh
cargo install diesel_cli --no-default-features --features postgres
```

Create database, and run migrations

```sh
diesel database setup
```
