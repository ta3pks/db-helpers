use std::{collections::HashMap, convert::Infallible};
use syn::{spanned::Spanned, Attribute};

use proc_macro2::{Span, TokenStream};

pub fn fail(span: Span, txt: impl AsRef<str>) -> Result<Infallible>
{
	Err(syn::Error::new(span, txt.as_ref()).to_compile_error())
}
pub struct AttrValue
{
	pub span: Span,
	pub value: String,
}
pub fn parse_attrs<'a>(
	i: &'a [Attribute],
	supported_attrs: &[&str],
) -> crate::Result<HashMap<String, AttrValue>>
//{{{
{
	let mut attrs = HashMap::new();
	for attr in i.iter().filter(|a| a.path.is_ident("table")) {
		{
			let meta = attr.parse_meta().map_err(|e| e.to_compile_error())?;
			let attr = match meta {
				//{{{
				syn::Meta::List(a) => {
					if a.nested.is_empty() {
						fail(
							a.path.span(),
							"table attribute must have at least one argument",
						)?;
					}
					a.nested
				}
				_ => {
					fail(
						attr.bracket_token.span,
						"table attribute parameters must be in table(key=\"value\",...) format",
					)?;
					unreachable!()
				}
			}; //}}}
			for attr in attr {
				match attr {
					//{{{
					syn::NestedMeta::Meta(syn::Meta::NameValue(a)) => {
						if !supported_attrs.iter().any(|v| a.path.is_ident(v)) {
							fail(
								a.path.span(),
								format!(
									"unsupported attribute: {}",
									a.path
										.get_ident()
										.map(|i| i.to_string())
										.unwrap_or_default()
								),
							)?;
						}
						let ident = a.path.get_ident().unwrap().to_string();
						let val = if let syn::Lit::Str(ref s) = a.lit {
							s.value()
						} else {
							fail(a.lit.span(), "only string values are supported")?;
							unreachable!()
						};
						attrs.insert(
							ident,
							AttrValue {
								span: a.span(),
								value: val,
							},
						);
					}
					//}}}
					_ => {
						fail(
							attr.span(),
							"table attribute parameters must be in table(key=\"value\",...) format",
						)?;
					}
				}
			}
		}
	}
	Ok(attrs)
	//}}}
}
pub type Result<T> = std::result::Result<T, TokenStream>;
