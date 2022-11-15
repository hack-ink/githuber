use super::*;
use crate::test_util::*;

#[tokio::test]
async fn list_repository_issues_should_work() {
	api_client().unwrap().get(list_repository_issues("hack-ink", "githuber")).await.unwrap();
}
