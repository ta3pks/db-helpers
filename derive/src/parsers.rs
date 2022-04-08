use crate::utils::parse_attrs;

use syn::{
	punctuated::Punctuated, spanned::Spanned, token::Comma, DataStruct, DeriveInput, Field, FieldsNamed, Ident
};

use crate::utils::fail;
pub struct ParsedRootMeta<'a>
{
	pub self_struct: &'a DataStruct,
	pub fields: &'a FieldsNamed,
	pub table_name: String,
	pub index: Option<String>,
}

pub fn parse_root(i: &DeriveInput) -> crate::Result<ParsedRootMeta>
//{{{
{
	let attrs = parse_attrs(&i.attrs, &["name", "index"])?; // top level
	match i.data {
		syn::Data::Struct(ref d) => match d.fields {
			syn::Fields::Named(ref f) => {
				if f.named.is_empty() {
					fail(i.ident.span(), "struct must have at least one named field")?;
					unreachable!()
				}
				Ok(ParsedRootMeta {
					self_struct: d,
					fields: f,
					table_name: attrs
						.get("name")
						.map(|v| v.value.clone())
						.unwrap_or_else(|| i.ident.to_string().to_lowercase()),
					index: attrs.get("index").map(|v| v.value.clone()),
				})
			}
			_ => {
				fail(i.ident.span(), "Only named struct fields are supported")?;
				unreachable!()
			}
		},
		_ => panic!("#[derive(Tbl)] can only be used on structs"),
	}
} //}}}

pub struct FieldInfo
{
	pub name: Ident,
	pub db_name: String,
	pub db_query: String,
}
pub fn parse_fields(f: &Punctuated<Field, Comma>) -> crate::Result<Vec<FieldInfo>>
{
	let mut fields = Vec::new();
	for field in f {
		let attrs = parse_attrs(&field.attrs, &["name", "q"])?;
		let ident = field
			.ident
			.as_ref()
			.ok_or_else(|| fail(field.span(), "Fields must have an identifier").unwrap_err())?;
		let name = attrs
			.get("name")
			.map(|v| v.value.clone())
			.unwrap_or_else(|| ident.to_string().to_lowercase());
		let q = if let Some(v) = attrs.get("q") {
			v.value.clone()
		} else {
			fail(
				field.span(),
				"Fields must have a query please add q=\"<YOUR QUERY>\"",
			)?;
			unreachable!()
		};
		fields.push(FieldInfo {
			name: ident.clone(),
			db_name: name,
			db_query: q,
		});
	}
	Ok(fields)
}
