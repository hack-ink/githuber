//! Commits related methods.
//!
//! GitHub reference(s):
//! - <https://docs.github.com/en/rest/commits/commits>

#[cfg(test)] mod test;

// hack-ink
use crate::prelude::*;

#[api_impl::api]
#[properties(
	category = "commits",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/commits"
)]
pub struct ListCommits<'a> {
	pub owner: &'a str,
	pub repo: &'a str,
	pub sha: Option<&'a str>,
	pub path: Option<&'a str>,
	pub author: Option<&'a str>,
	pub since: Option<&'a str>,
	pub until: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "commits",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/commits/{}/branches-where-head"
)]
pub struct ListBranchesForHeadCommit<'a> {
	pub owner: &'a str,
	pub repo: &'a str,
	pub commit_sha: &'a str,
}

#[api_impl::api]
#[properties(
	category = "commits",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/commits/{}/pulls"
)]
pub struct ListPullRequestsAssociatedWithACommit<'a> {
	pub owner: &'a str,
	pub repo: &'a str,
	pub commit_sha: &'a str,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "commits",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/commits/{}"
)]
pub struct GetACommit<'a> {
	pub owner: &'a str,
	pub repo: &'a str,
	pub r#ref: &'a str,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "commits",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/compare/{}"
)]
pub struct CompareTwoCommits<'a> {
	pub owner: &'a str,
	pub repo: &'a str,
	pub basehead: &'a str,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}
