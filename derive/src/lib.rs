use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse, DeriveInput};

#[proc_macro_derive(Table, attributes(table))]
pub fn parse_table(t: TokenStream) -> TokenStream
{
	let root: DeriveInput = parse(t).unwrap();
	let name = format_ident!("{}", root.ident.to_string().to_lowercase());
	let tbl_name = name.to_string();
	println!("{:#?}", &root);
	quote!(
		impl #tbl_name {
			fn table_name() -> &'static str {
		  #tbl_name
			}
		   }
	)
	.into()
}
