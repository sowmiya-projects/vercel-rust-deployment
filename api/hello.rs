// api/hello.rs

use vercel_runtime::{run, Body, Error, Request, Response};

async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::new("Hello Sowmiya 🚀 Rust deployed successfully!".into()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}