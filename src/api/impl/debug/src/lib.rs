/// Abstraction of GitHub REST API.
pub trait Api<'a> {
	/// GitHub REST APIs' base prefix.
	const BASE_URI: &'static str = "https://api.github.com";
	/// Request's header `Accept`'s value.
	const ACCEPT: &'static str;

	/// Request's target URI/URL.
	fn api(&self) -> String;
}

/// Extended GitHub REST API.
pub trait ApiExt<'a>: Api<'a> {
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
	/// HTTP GET method.
	Get,
	/// HTTP PATCH method.
	Patch,
	/// HTTP POST method.
	Post,
	/// HTTP PUT method.
	Put,
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
