//! GitHub REST API collections.

pub mod commits;
pub mod repos;

/// Abstraction of HTTP GET API.
pub trait ApiGet<'a> {
	/// GitHub REST APIs' base prefix.
	const BASE_URI: &'static str = "https://api.github.com";
	/// Request's header `Accept`'s value.
	const ACCEPT: &'static str;

	/// Request's query parameters.
	fn query_parameters(&self) -> Vec<(&'static str, String)>;

	/// Request's target URI/URL.
	fn api(&self) -> String;
}

/// Abstraction of HTTP POST API.
pub trait ApiPost<'a>: ApiGet<'a> {}
