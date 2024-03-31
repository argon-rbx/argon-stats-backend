use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
	run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
	Ok(Response::builder()
		.status(StatusCode::OK)
		.body("test".to_string().into())?)
}
