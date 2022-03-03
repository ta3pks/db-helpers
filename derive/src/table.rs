use super::{
	parsers::{self, parse_fields, FieldInfo, ParsedRootMeta}, pg
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput};

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
	let fields = parse_fields(&fields.named)?;
	let field_names = fields.iter().map(|v| v.db_name.clone());
	let pg_impls = pg::init(name, &table_name, &fields, &index);
	let getters = field_getters(&table_name, &fields);
	let non_snake_case: TokenStream = "#[allow(non_snake_case)]".parse().unwrap();
	Ok(quote!(
	  impl #name{
		  pub const fn __table_name() -> &'static str {
			  #table_name
		  }
			#non_snake_case
	  pub const fn __columns()->&'static[&'static str]{
			&[#(#field_names),*]
	  }
	#getters
	  }
	   #pg_impls
	))
}

fn field_getters(table_name: &str, fields: &[FieldInfo]) -> TokenStream
{
	let f = fields
		.iter()
		.map::<TokenStream, _>(|FieldInfo { name, db_name, .. }| {
			let name = name.to_string();
			format!(
				r#"
          #[allow(non_snake_case)]
          pub const fn __field_{name}()->&'static str{{"{table_name}.{db_name}"}}"#,
			)
			.parse()
			.unwrap()
		});
	quote!(#(#f)*)
}
