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

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}"
)]
pub struct GetARepository<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "PATCH",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}"
)]
pub struct UpdateARepository<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	pub name: Option<&'a str>,
	pub description: Option<&'a str>,
	pub homepage: Option<&'a str>,
	pub private: Option<bool>,
	pub visibility: Option<&'a str>,
	pub security_and_analysis: Option<&'a str>,
	pub has_issues: Option<bool>,
	pub has_projects: Option<bool>,
	pub has_wiki: Option<bool>,
	pub is_template: Option<bool>,
	pub default_branch: Option<&'a str>,
	pub allow_squash_merge: Option<bool>,
	pub allow_merge_commit: Option<bool>,
	pub allow_rebase_merge: Option<bool>,
	pub allow_auto_merge: Option<bool>,
	pub delete_branch_on_merge: Option<bool>,
	pub allow_update_branch: Option<bool>,
	pub use_squash_pr_title_as_default: Option<bool>,
	pub squash_merge_commit_title: Option<&'a str>,
	pub squash_merge_commit_message: Option<&'a str>,
	pub merge_commit_title: Option<&'a str>,
	pub merge_commit_message: Option<&'a str>,
	pub archived: Option<bool>,
	pub allow_forking: Option<bool>,
	pub web_commit_signoff_required: Option<bool>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "DELETE",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}"
)]
pub struct DeleteARepository<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "PUT",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/automated-security-fixes"
)]
pub struct EnableAutomatedSecurityFixes<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "DELETE",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/automated-security-fixes"
)]
pub struct DeleteAutomatedSecurityFixes<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/codeowners/errors"
)]
pub struct ListCodeownersError<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	pub r#ref: Option<&'a str>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/contributors"
)]
pub struct ListRepositoryContributors<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	pub anon: Option<bool>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "POST",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/dispatches"
)]
pub struct CreateARepositoryDispatchEvent<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	#[payload_ess_param]
	pub event_type: &'a str,
	// TODO: JSON?
	pub client_payload: Option<&'a str>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/languages"
)]
pub struct ListRepositoryLanguages<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/tags"
)]
pub struct ListRepositoryTags<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/teams"
)]
pub struct ListRepositoryTeams<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/topics"
)]
pub struct GetAllRepositoryTopics<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "PUT",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/topics"
)]
pub struct ReplaceAllRepositoryTopics<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	// TODO: JSON?
	#[payload_ess_param]
	pub names: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "POST",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/transfer"
)]
pub struct TransferARepository<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
	#[payload_ess_param]
	pub new_owner: &'a str,
	pub new_name: &'a str,
	// TODO: JSON?
	pub team_ids: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/vulnerability-alerts"
)]
pub struct CheckIfVulnerabilityAlertsAreEnabledForARepository<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "PUT",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/vulnerability-alerts"
)]
pub struct EnableVulnerabilityAlerts<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "DELETE",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/vulnerability-alerts"
)]
pub struct DisableVulnerabilityAlerts<'a> {
	#[path_param]
	pub owner: &'a str,
	#[path_param]
	pub repo: &'a str,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "POST",
	accept = "application/vnd.github+json",
	uri = "/repos/{}/{}/generate"
)]
pub struct CreateARepositoryUsingATemplate<'a> {
	#[path_param]
	pub template_owner: &'a str,
	#[path_param]
	pub template_repo: &'a str,
	pub owner: Option<&'a str>,
	#[payload_ess_param]
	pub name: &'a str,
	pub description: Option<&'a str>,
	pub include_all_branches: Option<bool>,
	pub private: Option<bool>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/repositories"
)]
pub struct ListPublicRepositories {
	pub since: Option<u32>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/user/repos"
)]
pub struct ListRepositoriesForTheAuthenticatedUser<'a> {
	pub visibility: Option<&'a str>,
	pub affiliation: Option<&'a str>,
	pub r#type: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
	pub since: Option<&'a str>,
	pub before: Option<&'a str>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "POST",
	accept = "application/vnd.github+json",
	uri = "/user/repos"
)]
pub struct CreateARepositoryForTheAuthenticatedUser<'a> {
	#[payload_ess_param]
	pub name: &'a str,
	pub description: Option<&'a str>,
	pub homepage: Option<&'a str>,
	pub private: Option<bool>,
	pub has_issues: Option<bool>,
	pub has_projects: Option<bool>,
	pub has_wiki: Option<bool>,
	pub has_discussions: Option<bool>,
	pub team_id: Option<u8>,
	pub auto_init: Option<bool>,
	pub gitignore_template: Option<&'a str>,
	pub license_template: Option<&'a str>,
	pub allow_squash_merge: Option<bool>,
	pub allow_merge_commit: Option<bool>,
	pub allow_rebase_merge: Option<bool>,
	pub allow_auto_merge: Option<bool>,
	pub delete_branch_on_merge: Option<bool>,
	pub squash_merge_commit_title: Option<&'a str>,
	pub squash_merge_commit_message: Option<&'a str>,
	pub merge_commit_title: Option<&'a str>,
	pub merge_commit_message: Option<&'a str>,
	pub has_downloads: Option<bool>,
	pub is_template: Option<bool>,
}

#[api_impl::api]
#[properties(
	category = "repos",
	method = "GET",
	accept = "application/vnd.github+json",
	uri = "/users/{}/repos"
)]
pub struct ListRepositoriesForAUser<'a> {
	#[path_param]
	pub username: &'a str,
	pub r#type: Option<&'a str>,
	pub sort: Option<&'a str>,
	pub direction: Option<&'a str>,
	pub per_page: Option<u8>,
	pub page: Option<u16>,
}
