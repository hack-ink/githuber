//! GitHub REST API collections.

pub mod commits;
pub mod issues;
pub mod repos;

/// Abstraction of GitHub REST API.
pub trait Api {
	/// GitHub REST APIs' base prefix.
	const BASE_URI: &'static str = "https://api.github.com";
	/// Request's header `Accept`'s value.
	const ACCEPT: &'static str;

	/// Request's target URI/URL.
	fn api(&self) -> String;
}

/// Extended GitHub REST API.
pub trait ApiExt: Api {
	/// HTTP method.
	const METHOD: Method;

	/// Request's payload parameters.
	///
	/// Can be one of:
	/// - Body parameters
	/// - Query parameters
	fn payload_params(&self) -> Vec<(&'static str, String)>;
}

/// HTTP methods.
pub enum Method {
	/// HTTP DELETE method.
	Delete,
	/// HTTP GET method.
	Get,
	/// HTTP PATCH method.
	Patch,
	/// HTTP POST method.
	Post,
	/// HTTP PUT method.
	Put,
}
