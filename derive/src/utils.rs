use std::convert::Infallible;

use proc_macro::TokenStream;
use proc_macro2::Span;

pub fn fail(span: Span, txt: impl AsRef<str>) -> Result<Infallible>
{
	Err(syn::Error::new(span, txt.as_ref())
		.to_compile_error()
		.into())
}
pub type Result<T> = std::result::Result<T, TokenStream>;
