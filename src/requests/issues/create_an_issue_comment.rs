//! GitHub API reference: https://docs.github.com/en/rest/reference/issues#create-an-issue-comment

// crates.io
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{Method as HttpMethod, Uri};
use serde::Serialize;
// hack-ink
use crate::{api, GithubApi};

#[derive(Clone, Debug, Default, DeriveBuilder)]
pub struct CreateAnIssueComment {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// integer	path
	/// issue_number parameter
	#[builder(setter(into))]
	pub issue_number: String,
	/// body	string	body
	/// Required. The contents of the comment.
	#[builder(setter(into))]
	pub body: String,
}
impl GithubApi<Vec<u8>> for CreateAnIssueComment {
	const HTTP_METHOD: HttpMethod = HttpMethod::POST;
	const PATH: &'static str = "/repos/{owner}/{repo}/issues/{issue_number}/comments";
	const ACCEPT: &'static str = "application/vnd.github.v3+json";

	fn build_uri(&self) -> Uri {
		api!(self, [owner, repo, issue_number]).parse().unwrap()
	}

	fn build_body(&self) -> Vec<u8> {
		serde_json::to_vec(&Body { body: &self.body }).unwrap()
	}
}

#[derive(Serialize)]
struct Body<'a> {
	body: &'a String,
}

#[test]
#[ignore]
fn create_an_issue_comment_should_work() {
	// crates.io
	use isahc::ReadResponseExt;
	use serde_json::Value;
	// hack-ink
	use crate::Githuber;

	let githuber = Githuber::from_env();
	let response = githuber
		.send_sync(
			CreateAnIssueCommentBuilder::default()
				.owner("AurevoirXavier")
				.repo("TEST")
				.issue_number("3")
				.body("TEST")
				.build()
				.unwrap(),
		)
		.unwrap()
		.json::<Value>()
		.unwrap();

	dbg!(response);
}
