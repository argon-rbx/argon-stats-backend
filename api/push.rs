use argon::{authorize, get_connection, get_query, ArgonStats};
use redis::Commands;
use vercel_runtime::{run, Body, Error, Request, RequestPayloadExt, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
	run(push).await
}

pub async fn push(request: Request) -> Result<Response<Body>, Error> {
	let query = get_query(&request);

	if let Some(fail) = authorize(&query) {
		return Ok(fail);
	}

	if let Some(payload) = request.payload::<ArgonStats>()? {
		let mut redis = get_connection()?;

		let mut stats: Vec<(&str, u64)> = payload.into_iter().collect();

		let existing_stats: Vec<u64> =
			redis.mget(stats.iter().map(|(stat, _)| *stat).collect::<Vec<&str>>())?;

		stats
			.iter_mut()
			.enumerate()
			.for_each(|(index, (_, value))| *value += existing_stats[index]);

		redis.mset(&stats)?;

		Ok(Response::builder()
			.status(StatusCode::OK)
			.body("Pushed stats successfully!".into())?)
	} else {
		Ok(Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body("Request has no payload!".into())?)
	}
}
