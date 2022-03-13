use std::{
	collections::HashMap, sync::Arc
};
use parking_lot::Mutex;

use super::{
	parsers::{self, parse_fields, FieldInfo, ParsedRootMeta}, pg
};
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput, Ident};
lazy_static! {
	pub(crate) static ref FIELD_MAP: Arc<Mutex<HashMap<String, String>>> = Default::default();
}

pub fn table(t: TokenStream) -> crate::Result<TokenStream>
{
	let root: DeriveInput = parse2(t).unwrap();
	let name = &root.ident;
	let ParsedRootMeta {
		fields,
		table_name,
		index,
		..
	} = parsers::parse_root(&root)?;
	{
		let mut map = FIELD_MAP.lock();
		map.insert(format!("{name}.__TABLE__"), table_name.to_string());
	}
	let fields = parse_fields(&fields.named)?;
	let field_names = fields.iter().map(|v| v.db_name.clone());
	let pg_impls = pg::init(name, &table_name, &fields, &index);
	field_getters(name, &fields);
	let non_snake_case: TokenStream = "#[allow(non_snake_case)]".parse().unwrap();
	Ok(quote!(
		impl #name{
		  #non_snake_case
			pub const fn __table_name() -> &'static str {
				#table_name
			}
			  #non_snake_case
		pub const fn __columns()->&'static[&'static str]{
			  &[#(#field_names),*]
		}
		}
	  impl db_helpers::legacy::Table for #name{
		fn table_name() -> String{
			#table_name.to_string()
	}
	}
		 #pg_impls
	  ))
}

fn field_getters(struct_name: &Ident, fields: &[FieldInfo])
{
	let mut meta = FIELD_MAP.lock();
	fields.iter().for_each(|FieldInfo { name, db_name, .. }| {
		meta.insert(format!("{struct_name}.{name}"), db_name.to_string());
	});
}
