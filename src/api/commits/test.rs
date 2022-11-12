use super::*;
use crate::test_util::*;

#[tokio::test]
async fn list_commits_should_work() {
	api_client().unwrap().get(list_commits("hack-ink", "githuber")).await.unwrap();
}

#[tokio::test]
async fn list_branches_for_head_commit_should_work() {
	api_client()
		.unwrap()
		.get(list_branches_for_head_commit("hack-ink", "githuber", "main"))
		.await
		.unwrap();
}

#[tokio::test]
async fn list_pull_requests_associated_with_a_commit_should_work() {
	api_client()
		.unwrap()
		.get(list_pull_requests_associated_with_a_commit("hack-ink", "githuber", "main"))
		.await
		.unwrap();
}

#[tokio::test]
async fn get_a_commit_should_work() {
	api_client().unwrap().get(get_a_commit("hack-ink", "githuber", "main")).await.unwrap();
}

#[tokio::test]
async fn compare_two_commits_should_work() {
	api_client().unwrap().get(compare_two_commits("hack-ink", "githuber", "main...archived")).await.unwrap();
}
