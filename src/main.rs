use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Request, Response, Server, Uri};

async fn dispatch(req: Request<Body>) -> Result<Response<Body>, Error> {
    println!("{}", req.uri());

    let client = Client::new();

    let url: Uri = "http://localhost:3000/static/js/bundle.js".parse().unwrap();
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
