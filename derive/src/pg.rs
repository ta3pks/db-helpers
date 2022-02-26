use crate::parsers::FieldInfo;
use proc_macro2::TokenStream;
use quote::quote;

use syn::Ident;

#[cfg(feature = "pg")]
pub fn init(
	struct_name: &Ident,
	table_name: &str,
	fields: &[FieldInfo],
	index: &Option<String>,
) -> TokenStream
{
	let from_row_ref = impl_from_row_ref(struct_name, fields);
	let create_table_str = pg_create_table_str(struct_name, table_name, fields);
	let impl_index = impl_index(struct_name, index);
	quote!(
	#create_table_str
	  #from_row_ref
	  #impl_index
	impl From<db_helpers::Row> for #struct_name {
	  fn from(row: db_helpers::Row) -> Self {
		Self::from(&row)
	  }
	}
	)
}
#[cfg(not(feature = "pg"))]
pub fn init(table_name: &str, fields: &Vec<FieldInfo>) -> TokenStream
{
	quote!()
}
fn pg_create_table_str(struct_name: &Ident, table_name: &str, fields: &[FieldInfo]) -> TokenStream
{
	let fields = fields
		.iter()
		.map(|f| format!(r#""{}" {}"#, f.db_name, f.db_query))
		.collect::<Vec<String>>()
		.join(",");
	let create_table_str = format!(r#"CREATE TABLE IF NOT EXISTS {table_name} ({fields})"#);
	quote! {
		impl #struct_name {
		  pub const fn __pg_create_table_str() -> &'static str {
			#create_table_str
		  }
	  }
	}
}
fn impl_from_row_ref(struct_name: &Ident, fields: &[FieldInfo]) -> TokenStream
{
	let f = fields.iter().map::<TokenStream, _>(|f| {
		let (k, v) = (&f.name, &f.db_name);
		quote! {#k: row.get(#v)}
	});
	quote! {
		impl From<&db_helpers::Row> for #struct_name {
		  fn from(row: &db_helpers::Row) -> Self {
			 {
		   Self{
		   #(#f),*
		   }
			}
		  }
		}
	}
}
fn impl_index(struct_name: &Ident, index: &Option<String>) -> TokenStream
{
	if let Some(i) = index {
		quote! {
		impl #struct_name {
			pub const fn __pg_index() -> &'static str {
				#i
			}
			}
		}
	} else {
		quote!(
		impl #struct_name {
			pub const fn __pg_index() -> &'static str {
		  ""
			}
			}

		)
	}
}
