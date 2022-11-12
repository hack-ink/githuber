//! Generate a GitHub API.

#![allow(clippy::tabs_in_doc_comments)]
#![deny(missing_docs)]

// proc-macro
use proc_macro::TokenStream;
// crates.io
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	*,
};

#[derive(Debug)]
enum ApiProperty {
	Category(String),
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
			"accept" => ApiProperty::Accept(value),
			"uri" => ApiProperty::Uri(value),
			_ => unreachable!(),
		})
	}
}

/// Generate a GitHub API.
///
/// # Example
/// ```ignore
/// use githuber::prelude::*;
///
/// #[api_impl::api]
/// #[properties(category = "repos", accept = "application/vnd.github+json", uri = "/orgs/{}/repos")]
/// pub struct ListOrganizationRepositories<'a> {
/// 	pub org: &'a str,
/// 	pub r#type: Option<&'a str>,
/// 	pub sort: Option<&'a str>,
/// 	pub direction: Option<&'a str>,
/// 	pub per_page: Option<u8>,
/// 	pub page: Option<u16>,
/// }
/// ```
#[proc_macro_attribute]
pub fn api(_: TokenStream, input: TokenStream) -> TokenStream {
	let api_struct = syn::parse_macro_input!(input as ItemStruct);

	// dbg!(&api_struct);

	let api_attrs = api_struct.attrs;

	// dbg!(&api_attrs);

	let api_name = api_struct.ident;
	let mut api_doc = String::new();
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
			ApiProperty::Accept(accept) => api_accept = accept,
			ApiProperty::Uri(uri) => api_uri = format!("{{}}{uri}"),
		});

	let api_vis = api_struct.vis;
	let api_generics = api_struct.generics;
	let mut api_fields = Vec::new();
	let mut api_ess_fields = Vec::new();
	let mut api_ess_fields_types = Vec::new();
	let mut api_opt_fields = Vec::new();
	let mut api_opt_fields_types = Vec::new();

	if let Fields::Named(fields) = api_struct.fields {
		fields.named.into_iter().for_each(|field| {
			api_fields.push(field.clone());

			if let Type::Path(path) = field.ty {
				if &path.path.segments[0].ident.to_string() == "Option" {
					api_opt_fields.push(field.ident);

					if let PathArguments::AngleBracketed(args) = &path.path.segments[0].arguments {
						if let GenericArgument::Type(ty) = &args.args[0] {
							api_opt_fields_types.push(ty.to_owned());
						}
					}
				}
			} else {
				api_ess_fields.push(field.ident);
				api_ess_fields_types.push(field.ty);
			}
		});
	}

	let api_opt_fields_names = api_opt_fields
		.iter()
		.map(|field| {
			field.as_ref().map(|field| field.to_string().trim_start_matches("r#").to_owned())
		})
		.collect::<Vec<_>>();
	let api_name_snake_case = format_ident!("{}", api_name.to_string().to_case(Case::Snake));

	quote::quote! {
		/// GitHub reference(s):
		#[doc = #api_doc]
		#[derive(Debug, Clone, PartialEq, Eq)]
		#api_vis struct #api_name #api_generics {
			#(
				#[allow(missing_docs)]
				#api_fields,
			)*
		}
		impl #api_generics #api_name #api_generics {
			#[doc = concat!("Build a [`", stringify!(#api_name), "`] instance.")]
			#api_vis fn new(#(#api_ess_fields: #api_ess_fields_types,)*) -> Self {
				Self {
					#(#api_ess_fields,)*
					#(#api_opt_fields: None,)*
				}
			}

			#(
				#[doc = concat!(
					"Set a new [`",
					stringify!(#api_ess_fields),
					"`](",
					stringify!(#api_name),
					"#structfield.",
					stringify!(#api_ess_fields),
					")."
				)]
				#api_vis fn #api_ess_fields(mut self, #api_ess_fields: #api_ess_fields_types) -> Self {
					self.#api_ess_fields = #api_ess_fields;

					self
				}
			)*

			#(
				#[doc = concat!(
					"Set a new [`",
					stringify!(#api_opt_fields),
					"`](",
					stringify!(#api_name),
					"#structfield.",
					stringify!(#api_opt_fields),
					")."
				)]
				#api_vis fn #api_opt_fields(mut self, #api_opt_fields: #api_opt_fields_types) -> Self {
					self.#api_opt_fields = Some(#api_opt_fields);

					self
				}
			)*
		}
		impl #api_generics ApiGet #api_generics for #api_name #api_generics {
			const ACCEPT: &'static str = #api_accept;

			fn query_parameters(&self) -> Vec<(&'static str, String)> {
				let mut query_parameters = Vec::new();

				#(
					if let Some(#api_opt_fields) = self.#api_opt_fields {
						query_parameters.push((#api_opt_fields_names, #api_opt_fields.to_string()));
					}
				)*

				query_parameters
			}

			fn api(&self) -> String {
				format!(#api_uri, Self::BASE_URI, #(self.#api_ess_fields,)*)
			}
		}
		#[doc = concat!("Build a [`", stringify!(#api_name), "`] instance.")]
		#api_vis fn #api_name_snake_case #api_generics(#(#api_ess_fields: #api_ess_fields_types,)*) -> #api_name #api_generics {
			#api_name::new(#(#api_ess_fields,)*)
		}
	}
	.into()
}
