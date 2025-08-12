#[path = "integration/users.rs"]
mod users;

use std::sync::OnceLock;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::core::Container;
use testcontainers_modules::testcontainers::runners::SyncRunner;

static PG_CONTAINER: OnceLock<Container<Postgres>> = OnceLock::new();
static USER: &'static str = "test_user";
static PASS: &'static str = "test_pass";
static DB_NAME: &'static str = "test_db";
static PG_DEFAULT_PORT: u16 = 5432;

#[ctor::ctor]
fn bootstrap_integration_tests() {
    let pg = Postgres::default()
        .with_user(USER)
        .with_password(PASS)
        .with_db_name(&DB_NAME)
        .start()
        .expect("Postgres test container to start successfully!");
    let connection_string = format!(
        "postgres://{USER}:{PASS}@127.0.0.1:{}/{DB_NAME}",
        pg.get_host_port_ipv4(PG_DEFAULT_PORT)
            .expect("Postgres test container should expose a host port")
    );
    std::env::set_var("DATABASE_URL", connection_string);
    PG_CONTAINER.set(pg).ok();
}
