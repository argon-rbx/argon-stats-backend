use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
	run(root).await
}

pub async fn root(_request: Request) -> Result<Response<Body>, Error> {
	Ok(Response::builder().status(StatusCode::OK).body(
		"Available Argon stats API endpoints:\n\n\
		/pull?stat={name?}\n\
		/push?auth={token}"
			.into(),
	)?)
}
