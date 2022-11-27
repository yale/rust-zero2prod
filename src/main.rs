use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config");

    let connection_pool =
        PgPool::connect_lazy(&config.database.connection_string().expose_secret())
            .expect("Failed to create a Postgres connection pool.");

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    run(listener, connection_pool)?.await
}
