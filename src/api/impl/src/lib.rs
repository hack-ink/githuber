//! Generate a modern ergonomic GitHub REST API.

#![allow(clippy::tabs_in_doc_comments)]
#![deny(missing_docs)]

// proc-macro
use proc_macro::TokenStream;
// crates.io
use convert_case::{Case, Casing};
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	*,
};

#[derive(Debug)]
enum ApiProperty {
	Category(String),
	Method(String),
	Accept(String),
	Uri(String),
}
impl Parse for ApiProperty {
	fn parse(input: ParseStream) -> Result<Self> {
		let name = input.parse::<Ident>()?.to_string();
		let value = if input.peek(Token![=]) {
			input.parse::<Token![=]>()?;

			if input.peek(LitStr) {
				input.parse::<LitStr>()?.value()
			} else {
				unreachable!()
			}
		} else {
			unreachable!()
		};

		Ok(match name.as_str() {
			"category" => ApiProperty::Category(value),
			"method" => ApiProperty::Method(value),
			"accept" => ApiProperty::Accept(value),
			"uri" => ApiProperty::Uri(value),
			_ => unreachable!(),
		})
	}
}

/// Generate a modern ergonomic GitHub REST API.
///
/// # Example
/// ```ignore
/// use githuber::prelude::*;
///
/// #[api_impl::api]
/// #[properties(
/// 	category = "repos",
/// 	method = "GET",
/// 	accept = "application/vnd.github+json",
/// 	uri = "/orgs/{}/repos"
/// )]
/// pub struct ListOrganizationRepositories<'a> {
/// 	#[path_param]
/// 	pub org: &'a str,
/// 	pub r#type: Option<&'a str>,
/// 	pub sort: Option<&'a str>,
/// 	pub direction: Option<&'a str>,
/// 	pub per_page: Option<u8>,
/// 	pub page: Option<u16>,
/// }
///
/// #[api_impl::api]
/// #[properties(
/// 	category = "repos",
/// 	method = "POST",
/// 	accept = "application/vnd.github+json",
/// 	uri = "/orgs/{}/repos"
/// )]
/// pub struct CreateAnOrganizationRepository<'a> {
/// 	#[path_param]
/// 	pub org: &'a str,
/// 	#[payload_ess_param]
/// 	pub name: &'a str,
/// 	pub description: Option<&'a str>,
/// 	pub homepage: Option<&'a str>,
/// 	pub private: Option<bool>,
/// 	pub visibility: Option<&'a str>,
/// 	pub has_issues: Option<bool>,
/// 	pub has_projects: Option<bool>,
/// 	pub has_wiki: Option<bool>,
/// 	pub has_downloads: Option<bool>,
/// 	pub is_template: Option<bool>,
/// 	pub team_id: Option<u8>,
/// 	pub auto_init: Option<bool>,
/// 	pub gitignore_template: Option<&'a str>,
/// 	pub license_template: Option<&'a str>,
/// 	pub allow_squash_merge: Option<bool>,
/// 	pub allow_rebase_merge: Option<bool>,
/// 	pub allow_auto_merge: Option<bool>,
/// 	pub delete_branch_on_merge: Option<bool>,
/// 	pub use_squash_pr_title_as_default: Option<bool>,
/// 	pub squash_merge_commit_title: Option<&'a str>,
/// 	pub squash_merge_commit_message: Option<&'a str>,
/// 	pub merge_commit_title: Option<&'a str>,
/// 	pub merge_commit_message: Option<&'a str>,
/// }
/// ```
#[proc_macro_attribute]
pub fn api(_: TokenStream, input: TokenStream) -> TokenStream {
	let api_struct = syn::parse_macro_input!(input as ItemStruct);

	// #[cfg(feature = "debug")]
	// dbg!(&api_struct);
	// #[cfg(feature = "debug")]
	// dbg!(&api_struct.fields);

	let api_attrs = api_struct.attrs;

	// #[cfg(feature = "debug")]
	// dbg!(&api_attrs);

	let api_name = api_struct.ident;
	let mut api_doc = String::new();
	let mut api_method = String::new();
	let mut api_accept = String::new();
	let mut api_uri = String::new();

	api_attrs
		.into_iter()
		.filter(|attr| attr.path.is_ident("properties"))
		.flat_map(|attr| {
			attr.parse_args_with(Punctuated::<ApiProperty, Token![,]>::parse_terminated)
				.unwrap()
				.into_iter()
		})
		.for_each(|property| match property {
			ApiProperty::Category(category) =>
				api_doc = format!(
					" - <https://docs.github.com/en/rest/{category}/{category}#{}>",
					api_name.to_string().to_case(Case::Kebab)
				),
			ApiProperty::Method(method) => api_method = method,
			ApiProperty::Accept(accept) => api_accept = accept,
			ApiProperty::Uri(uri) => api_uri = format!("{{}}{uri}"),
		});

	let api_vis = api_struct.vis;
	let api_generics = api_struct.generics;
	let mut api_path_params = Vec::new();
	let mut api_path_params_tys = Vec::new();
	let mut api_payload_ess_params = Vec::new();
	let mut api_payload_ess_params_tys = Vec::new();
	let mut api_payload_opt_params = Vec::new();
	let mut api_payload_opt_params_tys = Vec::new();

	if let Fields::Named(fields) = api_struct.fields {
		fields.named.into_iter().for_each(|field| {
			if field.attrs.is_empty() {
				if let Type::Path(path) = field.ty {
					if &path.path.segments[0].ident.to_string() == "Option" {
						api_payload_opt_params.push(field.ident);

						if let PathArguments::AngleBracketed(args) =
							&path.path.segments[0].arguments
						{
							if let GenericArgument::Type(ty) = &args.args[0] {
								api_payload_opt_params_tys.push(ty.to_owned());
							}
						}
					}
				}
			} else {
				match field.attrs[0].path.get_ident().unwrap().to_string().as_str() {
					"path_param" => {
						api_path_params.push(field.ident);
						api_path_params_tys.push(field.ty);
					},
					"payload_ess_param" => {
						api_payload_ess_params.push(field.ident);
						api_payload_ess_params_tys.push(field.ty);
					},
					_ => unreachable!(),
				}
			}
		});
	}

	let api_method = quote::format_ident!("{}", api_method.to_case(Case::Pascal));
	let get_names = |params: &[Option<Ident>]| {
		params
			.iter()
			.map(|field| {
				field.as_ref().map(|field| field.to_string().trim_start_matches("r#").to_owned())
			})
			.collect::<Vec<_>>()
	};
	let api_payload_ess_params_names = get_names(&api_payload_ess_params);
	let api_payload_opt_params_names = get_names(&api_payload_opt_params);
	let api_name_snake_case = quote::format_ident!("{}", api_name.to_string().to_case(Case::Snake));

	quote::quote! {
		/// GitHub reference(s):
		#[doc = #api_doc]
		#[derive(Debug, Clone, PartialEq, Eq)]
		#api_vis struct #api_name #api_generics {
			#(
				#[allow(missing_docs)]
				#api_path_params: #api_path_params_tys,
			)*
			#(
				#[allow(missing_docs)]
				#api_payload_ess_params: #api_payload_ess_params_tys,
			)*
			#(
				#[allow(missing_docs)]
				#api_payload_opt_params: Option<#api_payload_opt_params_tys>,
			)*
		}
		impl #api_generics #api_name #api_generics {
			#[doc = concat!(
				"Build a [`",
				stringify!(#api_name),
				"`] instance."
			)]
			#api_vis fn new(
				#(#api_path_params: #api_path_params_tys,)*
				#(#api_payload_ess_params: #api_payload_ess_params_tys,)*
			) -> Self {
				Self {
					#(#api_path_params,)*
					#(#api_payload_ess_params,)*
					#(#api_payload_opt_params: None,)*
				}
			}

			#(
				#[doc = concat!(
					"Set a new [`",
					stringify!(#api_path_params),
					"`](",
					stringify!(#api_name),
					"#structfield.",
					stringify!(#api_path_params),
					")."
				)]
				#api_vis fn #api_path_params(
					mut self,
					#api_path_params: #api_path_params_tys
				) -> Self {
					self.#api_path_params = #api_path_params;

					self
				}
			)*
			#(
				#[doc = concat!(
					"Set a new [`",
					stringify!(#api_payload_ess_params),
					"`](",
					stringify!(#api_name),
					"#structfield.",
					stringify!(#api_payload_ess_params),
					")."
				)]
				#api_vis fn #api_payload_ess_params(
					mut self,
					#api_payload_ess_params: #api_payload_ess_params_tys
				) -> Self {
					self.#api_payload_ess_params = #api_payload_ess_params;

					self
				}
			)*
			#(
				#[doc = concat!(
					"Set a new [`",
					stringify!(#api_payload_opt_params),
					"`](",
					stringify!(#api_name),
					"#structfield.",
					stringify!(#api_payload_opt_params),
					")."
				)]
				#api_vis fn #api_payload_opt_params(
					mut self,
					#api_payload_opt_params: #api_payload_opt_params_tys
				) -> Self {
					self.#api_payload_opt_params = Some(#api_payload_opt_params);

					self
				}
			)*
		}
		impl #api_generics Api #api_generics for #api_name #api_generics {
			const ACCEPT: &'static str = #api_accept;

			fn api(&self) -> String {
				format!(
					#api_uri,
					Self::BASE_URI,
					#(self.#api_path_params,)*
				)
			}
		}
		impl #api_generics ApiExt #api_generics for #api_name #api_generics {
			const METHOD: Method = Method::#api_method;

			fn payload_params(&self) -> Vec<(&'static str, String)> {
				let mut payload_params = vec![
					#((
						#api_payload_ess_params_names,
						self.#api_payload_ess_params.to_string()
					),)*
				];

				#(
					if let Some(#api_payload_opt_params) = self.#api_payload_opt_params {
						payload_params.push((
							#api_payload_opt_params_names,
							#api_payload_opt_params.to_string()
						));
					}
				)*

				payload_params
			}
		}
		#[doc = concat!(
			"Build a [`",
			stringify!(#api_name),
			"`] instance."
		)]
		#api_vis fn #api_name_snake_case #api_generics(
			#(#api_path_params: #api_path_params_tys,)*
			#(#api_payload_ess_params: #api_payload_ess_params_tys,)*
		) -> #api_name #api_generics {
			#api_name::new(
				#(#api_path_params,)*
				#(#api_payload_ess_params,)*
			)
		}
	}
	.into()
}
