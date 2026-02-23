use vercel_runtime::{run, Body, Error, Request, Response, service_fn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(handler);
    run(service).await
}

async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello Sowmiya 🚀 Rust deployed successfully!".into())?)
}
