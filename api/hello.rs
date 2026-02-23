use vercel_runtime::{run, Body, Error, Request, Response};

async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello Sowmiya 🚀 Rust deployed successfully!".into())?)
}

fn main() -> Result<(), Error> {
    run(handler)
}
