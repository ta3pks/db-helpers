use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn query(t: TokenStream) -> crate::Result<TokenStream>
{
	let t = t.to_string();
	let mut tokens = t
		.strip_prefix('"')
		.expect("expecting a string literal")
		.strip_suffix('"')
		.expect("expecting a string literal")
		.split(' ')
		.peekable();
	let mut q = Vec::new();
	while let Some(curr) = tokens.non_empty_next() {
		if !["::__TABLE__", "::{"].iter().any(|v| curr.contains(v)) {
			q.push(quote! {#curr});
			continue;
		}
		//handle table name
		if curr.contains("::__TABLE__") {
			let mut parts = curr.split("::__TABLE__");
			let struct_name = parts.next().unwrap();
			if struct_name.is_empty() {
				panic!("expecting a_valid_struct::__TABLE__  found {curr}");
			}
			let struct_name = format_ident!("{struct_name}");
			q.push(quote! {#struct_name::__table_name()});
			continue;
		}
		// handle fields
		let mut parts = curr.split("::");
		let struct_name = parts.next().unwrap();
		if struct_name.is_empty() {
			panic!(
				r#"expecting a_valid_struct::{{field,*}} found {curr}
TIP: 'struct_name::{{' part cannot contain spaces"#
			);
		}
		let fields = parts.next().unwrap();
		let mut all_fields = fields
			.to_string()
			.replace('{', "")
			.replace('}', "")
			.replace(' ', "")
			.split(',')
			.filter_map(|s| {
				if s.is_empty() {
					None
				} else {
					Some(s.to_string())
				}
			})
			.collect::<Vec<_>>();
		if !fields.ends_with('}') {
			//contains spaces
			let fields = tokens
				.collect_fields()
				.unwrap_or_else(|| panic!("missing closing '}}' for field names after {curr}"));
			all_fields.extend(fields.into_iter());
		}
		all_fields.iter().enumerate().for_each(|(i, field)| {
			dbg!(field);
			if field.starts_with('>') {
				let field = field.trim_start_matches('>');
				q.push(
					format!("{struct_name}::__field_with_table_{field}()")
						.parse()
						.unwrap(),
				);
			} else {
				q.push(format!("{struct_name}::__field_{field}()").parse().unwrap());
			}
			if i < all_fields.len() - 1 {
				q.push(quote!(","));
			}
		});
	}
	let q = q.into_iter();
	Ok(quote! {	  {
	  [
		#(#q),*
	].join(" ")
	}
	})
}

trait NonEmptyNext<'a>
{
	fn non_empty_next(&mut self) -> Option<&'a str>;
	fn collect_fields(&mut self) -> Option<Vec<String>>;
}
impl<'a, I: Iterator<Item = &'a str>> NonEmptyNext<'a> for I
{
	fn non_empty_next(&mut self) -> Option<&'a str>
	{
		for item in self {
			if !item.is_empty() {
				return Some(item);
			}
		}
		None
	}

	fn collect_fields(&mut self) -> Option<Vec<String>>
	{
		let mut fields = Vec::new();
		let mut found = false;
		while let Some(curr) = self.non_empty_next() {
			fields.extend(
				curr.to_string()
					.replace('}', "")
					.replace(' ', "")
					.split(',')
					.filter_map(|s| {
						if s.is_empty() {
							None
						} else {
							Some(s.to_string())
						}
					}),
			);
			if curr.ends_with('}') {
				found = true;
				break;
			}
		}
		if found && !fields.is_empty() {
			Some(fields)
		} else {
			None
		}
	}
}
#[cfg(test)]
mod tests
{
	#[test]
	fn it_works()
	{
		let cases = trybuild::TestCases::new();
		cases.compile_fail("tests/fail/*.rs");
		cases.pass("tests/success/*.rs");
	}
}
