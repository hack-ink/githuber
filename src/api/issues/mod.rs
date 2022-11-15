//! Issues related methods.
//!
//! GitHub reference(s):
//! - <https://docs.github.com/en/rest/issues/issues>

#[cfg(test)] mod test;

// hack-ink
use crate::prelude::*;

#[api_impl::api]
#[properties(
	category = "issues",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/issues"
)]
pub struct ListIssuesAssignedToTheAuthenticatedUser<'a> {
	pub filter: Option<&'a str>,
	pub state: Option<&'a str>,
	pub labels: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub since: Option<&'a str>,
	pub collab: Option<bool>,
	pub orgs: Option<bool>,
	pub owned: Option<bool>,
	pub pulls: Option<bool>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/orgs/{}/issues"
)]
pub struct ListOrganizationIssuesAssignedToTheAuthenticatedUser<'a> {
	#[path_param]
	pub org: &'a str,
	pub filter: Option<&'a str>,
	pub state: Option<&'a str>,
	pub labels: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub since: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/issues"
)]
pub struct ListRepositoryIssues<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	pub milestone: Option<&'a str>,
	pub state: Option<&'a str>,
	pub assignee: Option<&'a str>,
	pub creator: Option<&'a str>,
	pub mentioned: Option<&'a str>,
	pub labels: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub since: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "POST",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/issues"
)]
pub struct CreateAnIssue<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	#[payload_ess_param]
	pub title: &'a str,
	pub body: Option<&'a str>,
	// TODO: JSON?
	pub milestone: Option<&'a str>,
	// TODO: JSON?
	pub labels: Option<&'a str>,
	// TODO: JSON?
	pub assignees: Option<&'a str>,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/issues/{}"
)]
pub struct GetAnIssue<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	#[path_param]
	pub issue_number: u32,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "PATCH",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/issues/{}"
)]
pub struct UpdateAnIssue<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	#[path_param]
	pub issue_number: u32,
	pub title: Option<&'a str>,
	pub body: Option<&'a str>,
	pub state: Option<&'a str>,
	pub state_reason: Option<&'a str>,
	// TODO: JSON
	pub milestone: Option<&'a str>,
	// TODO: JSON
	pub labels: Option<&'a str>,
	// TODO: JSON
	pub assignees: Option<&'a str>,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "PUT",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/issues/{}/lock"
)]
pub struct LockAnIssue<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	#[path_param]
	pub issue_number: u32,
	pub lock_reason: Option<&'a str>,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "DELETE",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/issues/{}/lock"
)]
pub struct UnlockAnIssue<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	#[path_param]
	pub issue_number: u32,
}

#[api_impl::api]
#[properties(
	category = "issues",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/user/issues"
)]
pub struct ListUserAccountIssuesAssignedToTheAuthenticatedUser<'a> {
	pub filter: Option<&'a str>,
	pub state: Option<&'a str>,
	pub labels: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub since: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}
