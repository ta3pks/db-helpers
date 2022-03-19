use super::table::FIELD_MAP;
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};

pub fn query(t: TokenStream) -> crate::Result<TokenStream>
{
	let (tokens, _rest) = parse_query(t);
	let mut tokens = tokens.split(' ');

	let meta = FIELD_MAP.lock();
	let mut q = Vec::new();
	while let Some(curr) = tokens.non_empty_next() {
		if !["::__TABLE__", "::{"].iter().any(|v| curr.contains(v)) {
			q.push(curr.to_string());
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
			q.push(
				meta.get(&format!("{struct_name}.__TABLE__"))
					.unwrap_or_else(|| panic!("{struct_name} is not created with #[derive(Table)]"))
					.to_string(),
			);
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
		if all_fields.is_empty() {
			panic!("no field of {struct_name} specified perhaps you meant to use *");
		}
		all_fields.iter().enumerate().for_each(|(i, field)| {
			let table = meta
				.get(&format!("{struct_name}.__TABLE__"))
				.unwrap_or_else(|| panic!("{struct_name} is not created with #[derive(Table)]"));
			if field.starts_with('>') {
				let field = field.trim_start_matches('>');
				q.push(format!(
					"{table}.{}{}",
					meta.get(&format!("{struct_name}.{field}"))
						.unwrap_or_else(|| panic!("{struct_name} does not have field {field}")),
					if i < all_fields.len() - 1 { "," } else { "" }
				));
			} else {
				q.push(format!(
					r#""{}"{}"#,
					meta.get(&format!("{struct_name}.{field}"))
						.unwrap_or_else(|| panic!("{struct_name} does not have field {field}")),
					if i < all_fields.len() - 1 { "," } else { "" }
				));
			}
		});
	}
	let q = q.join(" ");
	if let Some(rest) = _rest {
		Ok(quote! {format!(#q #rest)})
	} else {
		Ok(quote! {#q})
	}
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

fn parse_query(t: TokenStream) -> (String, Option<TokenStream>)
{
	let mut tokens = t.into_iter();
	let q = if let Some(TokenTree::Literal(l)) = tokens.next() {
		let l = syn::parse2::<syn::Lit>(l.to_token_stream()).expect("expected literal");
		match l {
			syn::Lit::Str(s) => s.value(),
			_ => panic!("expected a string literal"),
		}
	} else {
		panic!("expected a string literal");
	};
	let rest = tokens.collect::<TokenStream>();
	if rest.is_empty() {
		(q, None)
	} else {
		(q, Some(rest))
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
