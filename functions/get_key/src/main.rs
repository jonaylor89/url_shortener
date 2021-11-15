
#[macro_use]
extern crate lambda_runtime as lambda;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate simple_logger;

use redis::Commands;

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    key: u32,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    // connect to redis
    let redis_addr = std::env::var("REDIS_ADDRESS").expect("redis endpoint");
    let redis_port = std::env::var("REDIS_PORT").expect("open redis port");
    let redis_passwd = std::env::var("REDIS_PASSWORD").expect("valid redis password");
    let redis_tls = std::env::var("REDIS_TLS").unwrap();

    let schema = match redis_tls.as_str() {
        "true" => "rediss",
        "false" => "redis",
        _ => "redis",
    };

    let conn_str = format!("{}://:{}@{}:{}", schema, redis_passwd, redis_addr, redis_port);

    let client = redis::Client::open(conn_str).expect("valid redis client");
    let mut conn = client.get_connection().expect("valid redis connection");

    match conn.get::<u32, String>(e.key) {
        Ok(url) => Ok(CustomOutput {
            url: url,
        }),
        Err(e) => Err(c.new_error(e.detail().unwrap_or_else(|| "invalid or expired link"))),
    }
}
