use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPool;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // telemetry
    let subscriber = get_subscriber(
        "zero2prod".into(), "info".into(), std::io::stdout
    );
    init_subscriber(subscriber);

    // database and server configs
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // start server
    run(listener, connection_pool)?.await
}


