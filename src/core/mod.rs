use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Request, Response, Server, Uri};
use std::net::SocketAddr;

async fn dispatch(req: Request<Body>) -> Result<Response<Body>, Error> {
    println!(
        "{}",
        req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
    );

    let client = Client::new();

    let uri_string = format!(
        "http://{}/{}",
        "localhost:3000",
        req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
    );

    let url: Uri = uri_string.parse().unwrap();
    let response = client.get(url).await?;

    Ok(response)
}

pub async fn run_gateway(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    // Create a server bound on the provided address
    let serve_future = Server::bind(&addr)
        // Serve requests using our `async serve_req` function.
        // `serve` takes a closure which returns a type implementing the
        // `Service` trait. `service_fn` returns a value implementing the
        // `Service` trait, and accepts a closure which goes from request
        // to a future of the response.
        .serve(make_service_fn(|_| async {
            {
                Ok::<_, hyper::Error>(service_fn(dispatch))
            }
        }));

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}
