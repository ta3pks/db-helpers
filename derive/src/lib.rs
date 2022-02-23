mod parsers;
mod utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput};

pub(crate) use utils::Result;
#[proc_macro_derive(Tbl, attributes(table))]
pub fn parse_table(t: TokenStream) -> TokenStream
{
	match run(t) {
		Ok(s) | Err(s) => s,
	}
}
fn run(t: TokenStream) -> Result<TokenStream>
{
	let root: DeriveInput = parse(t).unwrap();
	let name = &root.ident;
	let table_name = root.ident.to_string().to_lowercase();
	let (_s, _fields) = parsers::parse_root(&root)?;
	Ok(quote!(
	  impl #name{
		  fn table_name() -> &'static str {
			  #table_name
		  }
	  }
		impl db_helpers::Table for  #name {
		fn table_name() -> String {
			#name::table_name().to_string()
		}
		   }
	)
	.into())
}
