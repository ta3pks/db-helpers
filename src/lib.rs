pub mod query;
pub trait Table
{
	fn table_name() -> String;
}
