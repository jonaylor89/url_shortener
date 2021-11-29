#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate simple_logger;

use rand::{thread_rng, Rng};
use redis::Commands;

use lambda_http::{
    handler,
    http::{self, StatusCode},
    lambda_runtime::{self, Context, Error},
    Request, Response,
};

#[derive(Deserialize, Clone)]
struct CustomEvent {
    value: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    key: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda_runtime::run(handler(my_handler)).await?;

    Ok(())
}

async fn my_handler(request: Request, c: Context) -> http::Result<Response<CustomOutput>> {
    let body: CustomEvent = request.body();
    if body.value == "" {
        return Ok(Response::builder()
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Allow-Headers", "*")
            .status(StatusCode::BAD_REQUEST)
            .body(CustomOutput { key: 0_u32 }).unwrap());
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

    let conn_str = format!(
        "{}://:{}@{}:{}",
        schema, redis_passwd, redis_addr, redis_port
    );

    let client = redis::Client::open(conn_str).expect("valid redis client");
    let mut conn = client.get_connection().expect("valid redis connection");

    let key: u32 = thread_rng().gen();
    match conn.set::<u32, String, ()>(key, body.value) {
        Ok(_url) => Ok(Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "*")
            .status(StatusCode::OK)
            .body(CustomOutput { key: key }).unwrap()),
        Err(_e) => Ok(Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "*")
            .status(StatusCode::NOT_FOUND)
            .body(CustomOutput { key: 0_u32 }).unwrap()),
    }
}
