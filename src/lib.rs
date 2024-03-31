use redis::{Client, Connection, RedisResult};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use struct_util::{Get, Iter, Set};
use url::Url;
use vercel_runtime::{Body, Request, Response, StatusCode};

#[derive(Debug, Default, Serialize, Deserialize, Iter, Get, Set)]
pub struct ArgonStats {
	pub hours_used: u64,
	pub lines_synced: u64,
	pub files_synced: u64,
	pub sessions_started: u64,
	pub projects_built: u64,
}

pub fn get_query(request: &Request) -> HashMap<String, String> {
	let url = Url::parse(&request.uri().to_string()).unwrap();
	url.query_pairs().into_owned().collect()
}

pub fn get_connection() -> RedisResult<Connection> {
	let client = Client::open(env::var("REDIS_ADDR").unwrap())?;
	client.get_connection()
}

pub fn authorize(query: &HashMap<String, String>) -> Option<Response<Body>> {
	if let Some(auth) = query.get("auth") {
		if auth == &env::var("AUTH_TOKEN").unwrap() {
			return None;
		}
	}

	Some(
		Response::builder()
			.status(StatusCode::UNAUTHORIZED)
			.body("Unauthorized".to_string().into())
			.unwrap(),
	)
}
