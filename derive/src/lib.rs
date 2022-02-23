mod parsers;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Tbl, attributes(table))]
pub fn parse_table(t: TokenStream) -> TokenStream
{
	let root: DeriveInput = syn::parse_macro_input!(t);
	let name = &root.ident;
	let table_name = root.ident.to_string().to_lowercase();
	let (s, fields) = parsers::parse_root(&root.data);
	dbg!(s, fields);
	quote!(
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
	.into()
}
