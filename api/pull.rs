use argon::{get_connection, get_query, ArgonStats};
use redis::Commands;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
	run(pull).await
}

pub async fn pull(request: Request) -> Result<Response<Body>, Error> {
	let query = get_query(&request);

	let mut redis = get_connection()?;

	if let Some(stat) = query.get("stat") {
		let value: Option<u64> = redis.get(stat)?;

		if let Some(value) = value {
			Ok(Response::builder()
				.status(StatusCode::OK)
				.body(value.to_string().into())?)
		} else {
			Ok(Response::builder()
				.status(StatusCode::NOT_FOUND)
				.body(format!("`{}` stat not found!", stat).into())?)
		}
	} else {
		let dummy = ArgonStats::default();
		let keys: Vec<&str> = dummy.into_iter().map(|(key, _)| key).collect();

		let stats: Vec<u64> = redis.mget(&keys)?;

		let mut argon_stats = ArgonStats::default();

		keys.iter()
			.enumerate()
			.for_each(|(index, &stat)| argon_stats.set(stat, stats[index]).unwrap());

		Ok(Response::builder()
			.status(StatusCode::OK)
			.header("Access-Control-Allow-Origin", "https://argon.wiki")
			.body(json!(argon_stats).to_string().into())?)
	}
}
