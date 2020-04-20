use std::net::SocketAddr;

#[tokio::main]
pub async fn main() {
    println!("{}", armored_apis::say_hello());

    // Set the address to run our socket on.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    armored_apis::core::run_server(addr).await;
}
