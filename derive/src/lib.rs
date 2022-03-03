mod parsers;
mod pg;
mod query;
mod table;
mod utils;

pub(crate) use utils::Result;
#[proc_macro_derive(Table, attributes(table))]
pub fn table(t: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	match table::table(t.into()) {
		Ok(s) | Err(s) => s.into(),
	}
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Q(t: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	match query::query(t.into()) {
		Ok(s) | Err(s) => s.into(),
	}
}
