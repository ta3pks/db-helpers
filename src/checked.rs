pub trait FieldCheckable
{
	fn is_valid_column_name(name: &str) -> bool;
}
pub struct CheckableBounds
{
	start: Option<usize>,
	content_start: Option<usize>,
	content_end: Option<usize>,
	end: Option<usize>,
}

#[cfg(test)]
mod tests
{
	fn no_checkable_returns_as_is() {}
}
