use super::*;
use crate::test_util::*;

#[tokio::test]
async fn list_organization_repositories_should_work() {
	api_client().unwrap().get(list_organization_repositories("hack-ink")).await.unwrap();
}
