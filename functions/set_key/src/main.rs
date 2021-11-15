#[macro_use]
extern crate lambda_runtime as lambda;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate simple_logger;

use redis::Commands;
use rand::{thread_rng, Rng};

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    value: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    key: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.value == "" {
        error!("Empty key in request {}", c.aws_request_id);
        return Err(c.new_error("Empty key"));
    }
    // connect to redis
    let redis_addr = std::env::var("REDIS_ADDRESS").unwrap();
    let redis_port = std::env::var("REDIS_PORT").unwrap();
    let redis_passwd = std::env::var("REDIS_PASSWORD").unwrap();
    let redis_tls = std::env::var("REDIS_TLS").unwrap();

    let schema = match redis_tls.as_str() {
        "true" => "rediss",
        "false" => "redis",
        _ => "redis",
    };

    let conn_str = format!("{}://:{}@{}:{}", schema, redis_passwd, redis_addr, redis_port);

    let client = redis::Client::open(conn_str).expect("valid redis client");
    let mut conn = client.get_connection().expect("valid redis connection");

    let key: u32 = thread_rng().gen();
    match conn.set::<u32, String, ()>(key, e.value) {
        Ok(_url) => Ok(CustomOutput { key: key }),
        Err(_e) => Err(c.new_error("Error setting value")),
    }
}
