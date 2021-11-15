
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
    let redis_addr = std::env::var("REDIS_ADDRESS").unwrap();
    let client = redis::Client::open(redis_addr).unwrap();
    let mut conn = client.get_connection().unwrap();

    match conn.get::<u32, String>(e.key) {
        Ok(url) => Ok(CustomOutput {
            url: url,
        }),
        Err(_e) => Err(c.new_error("Invalid or expired link")),
    }
}
