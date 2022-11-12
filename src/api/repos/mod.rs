//! Commits related methods.
//!
//! GitHub reference(s):
//! - <https://docs.github.com/en/rest/repos>

#[cfg(test)] mod test;

// hack-ink
use crate::prelude::*;

#[api_impl::api]
#[properties(category = "repos", accept = "application/vnd.github+json", uri = "/orgs/{}/repos")]
pub struct ListOrganizationRepositories<'a> {
	pub org: &'a str,
	pub r#type: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}
