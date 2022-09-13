// crates.io
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Issue {
	pub number: u32,
}
