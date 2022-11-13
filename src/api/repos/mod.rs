//! Commits related methods.
//!
//! GitHub reference(s):
//! - <https://docs.github.com/en/rest/repos>

#[cfg(test)] mod test;

// hack-ink
use crate::prelude::*;

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/orgs/{}/repos"
)]
pub struct ListOrganizationRepositories<'a> {
	#[path_param]
	pub org: &'a str,
	pub r#type: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "POST",
	accept = "application/vnd.github+json",
	uri = "/orgs/{}/repos"
)]
pub struct CreateAnOrganizationRepository<'a> {
	#[path_param]
	pub org: &'a str,
	#[payload_ess_param]
	pub name: &'a str,
	pub description: Option<&'a str>,
	pub homepage: Option<&'a str>,
	pub private: Option<bool>,
	pub visibility: Option<&'a str>,
	pub has_issues: Option<bool>,
	pub has_projects: Option<bool>,
	pub has_wiki: Option<bool>,
	pub has_downloads: Option<bool>,
	pub is_template: Option<bool>,
	pub team_id: Option<u8>,
	pub auto_init: Option<bool>,
	pub gitignore_template: Option<&'a str>,
	pub license_template: Option<&'a str>,
	pub allow_squash_merge: Option<bool>,
	pub allow_rebase_merge: Option<bool>,
	pub allow_auto_merge: Option<bool>,
	pub delete_branch_on_merge: Option<bool>,
	pub use_squash_pr_title_as_default: Option<bool>,
	pub squash_merge_commit_title: Option<&'a str>,
	pub squash_merge_commit_message: Option<&'a str>,
	pub merge_commit_title: Option<&'a str>,
	pub merge_commit_message: Option<&'a str>,
}
