// crates.io
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{Method as HttpMethod, Uri};
// hack-ink
use crate::{uri, GithubApi};

#[derive(Clone, Debug, Default, DeriveBuilder)]
pub struct ListReleases {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// commit_sha	string	path
	/// commit_sha+ parameter
	#[builder(default)]
	pub per_page: Option<u32>,
	/// page	integer	query
	/// Page number of the results to fetch.
	#[builder(default)]
	pub page: Option<u32>,
}
impl GithubApi<()> for ListReleases {
	const HTTP_METHOD: HttpMethod = HttpMethod::GET;
	const PATH: &'static str = "/repos/{owner}/{repo}/releases";
	const ACCEPT: &'static str = "application/vnd.github.v3+json";

	fn build_uri(&self) -> Uri {
		uri!(self, [owner, repo], [per_page, page]).parse().unwrap()
	}

	fn build_body(&self) {}
}

#[test]
fn list_releases_should_work() {
	// crates.io
	use isahc::ReadResponseExt;
	use serde_json::Value;
	// hack-ink
	use crate::Githuber;

	let githuber = Githuber::from_env();
	let releases = githuber
		.send_sync(
			ListReleasesBuilder::default()
				.owner("l2ust")
				.repo("githuber")
				.build()
				.unwrap(),
		)
		.unwrap()
		.json::<Value>()
		.unwrap();

	dbg!(releases);
}
