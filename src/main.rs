use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Request, Response, Server, Uri};

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
    let mut response = client.get(url).await?;

    let body = hyper::body::to_bytes(response.body_mut()).await?;

    Ok(Response::new(Body::from(body)))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{}", armored_apis::say_hello());

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Error>(service_fn(dispatch)) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;
    Ok(())
}
