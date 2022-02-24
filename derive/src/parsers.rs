use std::collections::HashMap;

use syn::{spanned::Spanned, DataStruct, DeriveInput, FieldsNamed};

use crate::utils::fail;

pub fn parse_root(i: &DeriveInput) -> crate::Result<(&DataStruct, &FieldsNamed, String)>
//{{{
{
	let attrs = parse_attrs(i, &["name"])?; // top level
	match i.data {
		syn::Data::Struct(ref d) => match d.fields {
			syn::Fields::Named(ref f) => {
				if f.named.is_empty() {
					fail(i.ident.span(), "struct must have at least one named field")?;
					unreachable!()
				}
				Ok((
					d,
					f,
					attrs
						.get("name")
						.cloned()
						.unwrap_or_else(|| i.ident.to_string().to_lowercase()),
				))
			}
			_ => {
				fail(i.ident.span(), "Only named struct fields are supported")?;
				unreachable!()
			}
		},
		_ => panic!("#[derive(Tbl)] can only be used on structs"),
	}
} //}}}

fn parse_attrs(i: &DeriveInput, supported_attrs: &[&str])
	-> crate::Result<HashMap<String, String>>
//{{{
{
	let mut attrs = HashMap::new();
	for attr in i.attrs.iter().filter(|a| a.path.is_ident("table")) {
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
						let val = if let syn::Lit::Str(s) = a.lit {
							s.value()
						} else {
							fail(a.lit.span(), "only string values are supported")?;
							unreachable!()
						};
						attrs.insert(ident, val);
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
