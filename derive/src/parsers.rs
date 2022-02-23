use syn::{DataStruct, DeriveInput, FieldsNamed};

use crate::utils::fail;

pub fn parse_root(i: &DeriveInput) -> crate::Result<(&DataStruct, &FieldsNamed)>
//{{{
{
	let _attrs = parse_attrs(i, &["name"])?; // top level
	match i.data {
		syn::Data::Struct(ref d) => match d.fields {
			syn::Fields::Named(ref f) => {
				if f.named.is_empty() {
					fail(i.ident.span(), "struct must have at least one named field")?;
					unreachable!()
				}
				Ok((d, f))
			}
			_ => {
				fail(i.ident.span(), "Only named struct fields are supported")?;
				unreachable!()
			}
		},
		_ => panic!("#[derive(Tbl)] can only be used on structs"),
	}
} //}}}

fn parse_attrs(i: &DeriveInput, _supported_attrs: &[&str]) -> crate::Result<()>
//{{{
{
	for attr in i.attrs.iter().filter(|a| a.path.is_ident("table")) {
		{
			//let tokens = syn::parse2(attr.tokens);
			fail(
				attr.bracket_token.span,
				"table attribute requires parameters please use table([key=\"value\",...])",
			)?;
		}
	}
	Ok(())
} //}}}
