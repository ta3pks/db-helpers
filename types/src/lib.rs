pub trait Table
{
	fn table_name() -> String;
}
pub trait Index
{
	fn index() -> Option<String>
	{
		None
	}
}
