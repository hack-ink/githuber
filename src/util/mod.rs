// --- std ---
#[cfg(test)]
use std::env;
// --- githuber ---
#[cfg(test)]
use crate::Githuber;

#[macro_export]
macro_rules! api {
	($self:ident, $([$($path_part:ident),+])?) => {{
		format!(
			"{}{}",
			$crate::Githuber::API_BASE_URL,
			Self::PATH
				$($(
					.replace($crate::api!($self, $path_part), &$self.$path_part)
				)+)?
		)
	}};
	($self:ident, r#ref) => {
		"{ref}"
	};
	($self:ident, $path_part:ident) => {
		&format!("{{{}}}", stringify!($path_part))
	};
}

#[macro_export]
macro_rules! queries {
	($self:ident, $([$($query:ident),+])?) => {{
		#[allow(unused_mut)]
		let mut queries = ::std::string::String::new();

		$($(
			$crate::queries!($self, $query, queries);
		)+)?

		queries.trim_end_matches('&').to_owned()
	}};
	($self:ident, r#ref, $queries:expr) => {
		if let Some(r#ref) = &$self.r#ref {
			$queries.push_str(&format!("ref={}&", r#ref));
			}
	};
	($self:ident, $query:ident, $queries:expr) => {
		if let Some($query) = &$self.$query {
			$queries.push_str(&format!("{}={}&", stringify!($query), $query));
			}
	};
}

#[macro_export]
macro_rules! uri {
	($self:ident, $([$($path_part:ident),+])?$(, [$($query:ident),+])?) => {{
		let api = $crate::api!($self, $([$($path_part),+])?);
		let queries = $crate::queries!($self, $([$($query),+])?);

		if queries.is_empty() {
			api
		} else {
			format!("{}?{}", api, queries)
		}
	}};
}

#[cfg(test)]
impl Githuber {
	pub fn from_env() -> Self {
		let _ = pretty_env_logger::try_init();

		Githuber::new(env::var("GITHUB_OAUTH_TOKEN").expect("Expect `GITHUB_OAUTH_TOKEN`"))
	}
}

#[cfg(test)]
mod tests {
	// --- crates.io ---
	use isahc::http::Uri;

	#[derive(Clone, Debug, Default)]
	pub struct Api {
		pub a: String,
		pub b: String,
		pub c: String,
		pub d: Option<String>,
		pub e: Option<String>,
		pub f: Option<String>,
	}
	impl Api {
		const PATH: &'static str = "/{a}/{b}/{c}";

		fn api(&self) -> Uri {
			crate::api!(self, [a, b, c]).parse().unwrap()
		}

		fn queries(&self) -> String {
			crate::queries!(self, [d, e, f])
		}

		fn uri(&self) -> String {
			crate::uri!(self, [a, b, c], [d, e, f])
		}
	}

	#[test]
	fn api_should_work() {
		assert_eq!(
			Api {
				a: "path_segment_a".into(),
				b: "path_segment_b".into(),
				c: "path_segment_c".into(),
				..Default::default()
			}
			.api(),
			"https://api.github.com/path_segment_a/path_segment_b/path_segment_c"
		);
	}

	#[test]
	fn queries_should_work() {
		assert_eq!(
			Api {
				d: Some("query_d".into()),
				e: Some("query_e".into()),
				f: Some("query_f".into()),
				..Default::default()
			}
			.queries(),
			"d=query_d&e=query_e&f=query_f".to_string()
		);
	}

	#[test]
	fn uri_should_work() {
		assert_eq!(
			Api {
				a: "path_segment_a".into(),
				b: "path_segment_b".into(),
				c: "path_segment_c".into(),
				d: Some("query_d".into()),
				e: Some("query_e".into()),
				f: Some("query_f".into()),
			}
			.uri(),
			"https://api.github.com/path_segment_a/path_segment_b/path_segment_c?d=query_d&e=query_e&f=query_f"
		);
	}
}
