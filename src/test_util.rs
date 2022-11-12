// std
use std::env;
// crates.io
use anyhow::Result;
use reqwest::{
	header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT},
	Client, ClientBuilder,
};
use serde_json::Value;
// hack-ink
use crate::prelude::*;

pub struct ApiClient(Client);
impl ApiClient {
	pub async fn get<'a, R>(&self, request: R) -> Result<Value>
	where
		R: ApiGet<'a>,
	{
		let response = self
			.0
			.get(request.api())
			.header(ACCEPT, R::ACCEPT)
			.query(&request.query_parameters())
			.send()
			.await?
			.json::<Value>()
			.await?;

		tracing::trace!("{:?}", response);

		Ok(response)
	}
}

pub fn api_client() -> Result<ApiClient> {
	let _ = tracing_subscriber::fmt::try_init();

	Ok(ApiClient(
		ClientBuilder::new()
			.default_headers(HeaderMap::from_iter([
				(USER_AGENT, "GitHuber-0.4.0".parse()?),
				(AUTHORIZATION, env::var("GITHUB_TOKEN")?.parse()?),
			]))
			.build()?,
	))
}