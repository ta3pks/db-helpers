mod parsers;
mod pg;
mod utils;

use parsers::{parse_fields, ParsedRootMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput};

pub(crate) use utils::Result;
#[proc_macro_derive(Tbl, attributes(table))]
pub fn parse_table(t: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	match run(t.into()) {
		Ok(s) | Err(s) => s.into(),
	}
}
fn run(t: TokenStream) -> Result<TokenStream>
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
	Ok(quote!(
	  impl #name{
		  pub const fn __table_name() -> &'static str {
			  #table_name
		  }
	  pub fn __columns()->&'static[&'static str]{
			&[#(#field_names),*]
	  }
	  }
		impl db_helpers::Table for  #name {
		fn table_name() -> String {
			#name::__table_name().to_string()
		}
		   }
	   #pg_impls
	))
}
