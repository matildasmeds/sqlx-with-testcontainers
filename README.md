# Setup example for sqlx with testcontainers

This example shows how to setup integration tests for [sqlx](https://docs.rs/sqlx/latest/sqlx/), using the [testcontainers crate](https://docs.rs/testcontainers/latest/testcontainers/).

## Prerequisites

- Docker daemon is [installed](https://docs.docker.com/get-started/get-docker/) and running
- Network access to pull images and dependencies
- Rust version 1.85+

## Running the tests

The setup uses sqlx offline mode for compile time checks, which also works well with CI pipelines.
The actual tests are executed against a containerized database instance, in this case, Postgres.

Clone the project on your machine, then run:
`SQLX_OFFLINE=true cargo test`

Testcontainers launches a Postgres container and [sqlx::test](https://docs.rs/sqlx/latest/sqlx/attr.test.html) injects the connection pool into the tests. It should be easy to adapt this to MySQL/MariaDB. If you use SQLite, you won't need a container for it, just use sqlx::test direclty.

## Troubleshooting

Here are some typical error situations that might occur when you run this code, or adapt it to your project.

`Client(Init(SocketNotFoundError("/var/run/docker.sock")))`
Start Docker and try again.

`SQLX_OFFLINE=true but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
For the offline mode to work, we need to have cached json data created (by cargo sqlx prepare) and committed to the repository. This repository comes with the prepared data, but this could occur when setting up your own project. You would need to start a postgres container locally, and then export DATABASE_URL with the correct connection string. After that you can run `cargo sqlx prepare` as the error message suggests. I recommend using offline mode for the test suite, and also committing the cached query data to the repository.

`error: error communicating with database: Connection refused (os error 61)`
This could happen if `SQLX_OFFLINE` is not set, and no database is available for compilation checks.

## Guarantees

Well, there are not much guarantees, this is the internet! But, if you find that something needs fixing or is not working, please let me know. I'll buy you a coffee!
